use crate::byday::ByDay;
use crate::dt_prop::local_datetime_with_tz;
use crate::freq::Frequency;
use crate::mappings::{self};
use crate::recur::Recur;
use crate::rrule::RRule;
use crate::util::{days_in_year, is_leap_year};
use crate::weekday::{days_until, Weekday};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone, Timelike, Utc};
use chrono_tz::Tz;

mod daily;
mod hms_ly;
mod monthly;
mod weekly;
mod yearly;

pub struct RRuleIter {
    // recurrence rules
    recur: Recur,

    // DTSTART as DateTime<Tz>
    // If DTSTART is floating the timezone is UTC
    dt_start: DateTime<Tz>,

    // Timezone of DTSTART, None if DTSTART is floating
    dt_start_tz: Option<Tz>,

    until: Option<DateTime<Utc>>,

    interval: u32,
    count: Option<u32>,
    week_start: Weekday,

    done: bool,

    step: fn(&mut RRuleIter),

    hours: Vec<u32>,
    minutes: Vec<u32>,
    seconds: Vec<u32>,

    // progress tracking
    year: i32,
    days_idx: usize,
    hours_idx: usize,
    minutes_idx: usize,
    seconds_idx: usize,

    // Use depends on freq
    //
    // - Yearly: Not used
    //
    // - Monthly, Weekly and Daily:
    //   Counts each recurrence BEFORE interval.
    //   Frequency units (months, weeks, days) where nth_unit % interval != 0 get skipped
    //
    // - Hourly, Minutely tracks the nth occurrence to filter over BYSETPOS
    nth_unit: u64,

    // vector of year days
    //
    // example:
    // self.year = 2020
    //  -1 = 31.12.2019
    //   0 = 01.01.2020
    //   1 = 02.01.2020
    // 366 = 01.01.2021
    //
    // Rebuilt for each year the iterator steps into
    days: Vec<i32>,
}

#[derive(Debug, Clone, Copy)]
pub enum RRuleIterYield {
    DateTimeLocal(NaiveDateTime),
    DateTimeTz(DateTime<Tz>),
}

impl RRuleIter {
    pub fn new(rrule: &RRule) -> Self {
        let dt_start = rrule.dt_start.0.to_datetime();
        let dt_start_tz = rrule.dt_start.0.result_tz();

        let until = rrule.recur.until.map(|until| match until {
            crate::dt::Dt::Date(date) => Utc.from_utc_datetime(&date.and_hms(0, 0, 0)),
            crate::dt::Dt::DateTimeLocal(datetime) => Utc.from_utc_datetime(&datetime),
            crate::dt::Dt::DateTimeUtc(datetime) => datetime,
        });

        let mut recur = rrule.recur.clone();
        recur.sort_and_dedup();

        // Initialize hours vec
        let hours = if recur.by_hour.is_empty() {
            if matches!(
                recur.freq,
                Frequency::Hourly | Frequency::Minutely | Frequency::Secondly
            ) {
                (0..24).collect()
            } else {
                vec![dt_start.time().hour()]
            }
        } else {
            recur.by_hour.clone()
        };

        // Initialize minutes vec
        let minutes = if recur.by_minute.is_empty() {
            if matches!(recur.freq, Frequency::Minutely | Frequency::Secondly) {
                (0..60).collect()
            } else {
                vec![dt_start.time().minute()]
            }
        } else {
            recur.by_minute.clone()
        };

        // Initialize seconds vec
        let seconds = if recur.by_second.is_empty() {
            if matches!(recur.freq, Frequency::Secondly) {
                (0..60).collect()
            } else {
                vec![dt_start.time().second()]
            }
        } else {
            recur.by_second.clone()
        };

        // Set the highest possible step function to skip some checks.
        // TODO: not sure if this is worth it, compiler might be smarter than this
        let step = if seconds.len() == 1 {
            if minutes.len() == 1 {
                if hours.len() == 1 {
                    step_day
                } else {
                    step_hour
                }
            } else {
                step_minute
            }
        } else {
            step_second
        };

        let mut this = Self {
            recur,

            dt_start,
            dt_start_tz,

            until,

            interval: rrule.recur.interval.unwrap_or(1),
            count: rrule.recur.count,
            week_start: rrule.recur.week_start.unwrap_or(Weekday::Monday),

            done: false,

            step,

            hours,
            minutes,
            seconds,

            year: dt_start.year(),
            days_idx: 0,
            hours_idx: 0,
            minutes_idx: 0,
            seconds_idx: 0,

            nth_unit: 0,

            days: vec![],
        };

        // build days vector
        this.rebuild_days();

        // step to first DTSTART time
        let dt_start_hour = dt_start.hour();
        let dt_start_minute = dt_start.minute();
        let dt_start_second = dt_start.second();

        if let Some(hours_idx) = this.hours.iter().position(|&hour| hour >= dt_start_hour) {
            if let Some(minutes_idx) = this
                .minutes
                .iter()
                .position(|&minute| minute >= dt_start_minute)
            {
                if let Some(seconds_idx) = this
                    .seconds
                    .iter()
                    .position(|&second| second >= dt_start_second)
                {
                    this.hours_idx = hours_idx;
                    this.minutes_idx = minutes_idx;
                    this.seconds_idx = seconds_idx;
                    return this;
                }
            }
        }

        // FALLTHROUGH: DTSTART time doesn't match for the day - step one
        step_day(&mut this);
        this
    }

    fn rebuild_days(&mut self) {
        self.days.clear();

        match self.recur.freq {
            Frequency::Secondly => self.rebuild_days_hms_ly(),
            Frequency::Minutely => self.rebuild_days_hms_ly(),
            Frequency::Hourly => self.rebuild_days_hms_ly(),
            Frequency::Daily => self.rebuild_days_daily(),
            Frequency::Weekly => self.rebuild_days_weekly(),
            Frequency::Monthly => self.rebuild_days_monthly(),
            Frequency::Yearly => self.rebuild_days_yearly(),
        }

        self.days.sort_unstable();
        self.days.dedup();
    }

    fn by_day_allows(&self, month1: u32, yearday: i32) -> bool {
        assert!(yearday >= 0);

        if self.recur.by_day.is_empty() {
            true
        } else {
            self.recur.by_day.iter().any(|by_day| {
                // this can be solved without iteration
                // via ((yearday - base_offset) % 7) == 0 or something
                by_day
                    .days_in_month(self.year, month1)
                    .any(|yd| yd == yearday)
            })
        }
    }

    fn to_yield(&self, datetime: NaiveDateTime) -> Option<RRuleIterYield> {
        if let Some(tz) = self.dt_start_tz {
            let datetime = local_datetime_with_tz(datetime, tz);

            if let Some(until) = self.until {
                if until < datetime {
                    return None;
                }
            }

            Some(RRuleIterYield::DateTimeTz(datetime))
        } else {
            if let Some(until) = self.until {
                if until.naive_local() < datetime {
                    return None;
                }
            }

            Some(RRuleIterYield::DateTimeLocal(datetime))
        }
    }
}

impl Iterator for RRuleIter {
    type Item = RRuleIterYield;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        if let Some(count) = &mut self.count {
            if *count == 0 {
                return None;
            } else {
                *count -= 1;
            }
        }

        // TODO this panic protection is insufficient
        // as rules like every 29th Feb on a Sunday
        // step multiple years inside a single `step` call
        if self.year == chrono::MAX_DATE.year() - 1 {
            return None;
        }

        let second = self.seconds[self.seconds_idx];
        let minute = self.minutes[self.minutes_idx];
        let hour = self.hours[self.hours_idx];

        let mut yd = self.days[self.days_idx];

        let this_year_len = days_in_year(self.year) as i32;

        let (year, leap_year) = if yd < 0 {
            let year = self.year - 1;
            let leap_year = is_leap_year(year);

            if leap_year {
                yd += 366;
            } else {
                yd += 365;
            }

            (year, leap_year)
        } else if yd >= this_year_len {
            let year = self.year + 1;
            yd -= this_year_len;
            (year, is_leap_year(year))
        } else {
            (self.year, is_leap_year(self.year))
        };

        let (month, day) = if leap_year {
            (
                mappings::YEARDAY_TO_MONTH_LEAPYEAR[yd as usize],
                mappings::YEARDAY_TO_DAY_LEAPYEAR[yd as usize],
            )
        } else {
            (
                mappings::YEARDAY_TO_MONTH_NORMAL[yd as usize],
                mappings::YEARDAY_TO_DAY_NORMAL[yd as usize],
            )
        };

        let datetime =
            NaiveDate::from_ymd(year, month as u32, day as u32).and_hms(hour, minute, second);

        let ret = self.to_yield(datetime)?;

        (self.step)(self);

        Some(ret)
    }
}

fn step_second(this: &mut RRuleIter) {
    if this.recur.freq == Frequency::Secondly {
        this.seconds_idx += this.interval as usize;
    } else {
        this.seconds_idx += 1;
    }

    while this.seconds_idx >= this.seconds.len() {
        this.seconds_idx -= this.seconds.len();
        step_minute(this);
    }
}

fn step_minute(this: &mut RRuleIter) {
    if this.recur.freq == Frequency::Minutely {
        this.minutes_idx += this.interval as usize;
    } else {
        this.minutes_idx += 1;
    }

    while this.minutes_idx >= this.minutes.len() {
        this.minutes_idx -= this.minutes.len();
        step_hour(this);
    }
}

fn step_hour(this: &mut RRuleIter) {
    if this.recur.freq == Frequency::Hourly {
        this.hours_idx += this.interval as usize;
    } else {
        this.hours_idx += 1;
    }

    while this.hours_idx >= this.hours.len() {
        this.hours_idx -= this.hours.len();
        step_day(this);
    }
}

fn step_day(this: &mut RRuleIter) {
    this.days_idx += 1;

    if this.days_idx == this.days.len() {
        this.days_idx = 0;

        if this.recur.freq == Frequency::Yearly {
            this.year += this.interval as i32;
        } else {
            this.year += 1;
        }

        this.rebuild_days();
    }
}

// =========================
// Helper utilities

fn months(by_month: &[u32]) -> impl Iterator<Item = u32> + '_ {
    if by_month.is_empty() {
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12].iter().copied()
    } else {
        by_month.iter().copied()
    }
}

fn days<'a>(
    year: i32,
    month1: u32,
    by_month_day: &'a [i32],
    by_day: &'a [ByDay],
) -> impl Iterator<Item = u32> + 'a {
    let days_in_month =
        mappings::days_in_month(year, month1 - 1).expect("only valid months from 1-12");

    (0..days_in_month).filter_map(move |day0| {
        let day1 = day0 + 1;

        // filter BYMONTHDAY
        if !by_month_day.is_empty()
            && !by_month_day.iter().any(|&monthday| {
                if monthday > 0 {
                    monthday as u32 == day1
                } else if monthday < 0 && -monthday < days_in_month as i32 {
                    let monthday = (days_in_month as i32 - monthday) as u32;

                    monthday as u32 == day1
                } else {
                    false
                }
            })
        {
            return None;
        }

        let date = NaiveDate::from_ymd_opt(year, month1, day1)?;

        let weekday = date.weekday();

        // filter BYDAY
        if !by_day.is_empty()
            && !by_day.iter().any(|byday| {
                if let ByDay::All(wd) = byday {
                    *wd == weekday
                } else {
                    false
                }
            })
        {
            return None;
        }

        Some(date.ordinal0())
    })
}

struct WeeksInYear {
    yd_of_first_weekday: i32,
    weeks: i32,
}

fn weeks_in_year(year: i32, week_start: chrono::Weekday) -> WeeksInYear {
    let year_len = days_in_year(year) as i32;

    let first_day_of_this_year = NaiveDate::from_yo(year, 1);
    let first_day_of_next_year = NaiveDate::from_yo(year + 1, 1);

    let wkd_of_first_day_this_year = first_day_of_this_year.weekday();
    let wkd_of_first_day_next_year = first_day_of_next_year.weekday();

    let mut wkst_offset_this_year =
        days_until(wkd_of_first_day_this_year as u32, week_start as u32) as i32;

    let mut wkst_offset_next_year =
        days_until(wkd_of_first_day_next_year as u32, week_start as u32) as i32;

    if wkst_offset_this_year >= 4 {
        wkst_offset_this_year -= 7;
    }

    if wkst_offset_next_year >= 4 {
        wkst_offset_next_year += year_len - 7;
    } else {
        wkst_offset_next_year += year_len;
    }

    let first_yd_of_first_week_this_year = wkst_offset_this_year;
    let first_yd_of_first_week_next_year = wkst_offset_next_year;

    debug_assert_eq!(
        (first_yd_of_first_week_next_year - first_yd_of_first_week_this_year) % 7,
        0
    );

    let weeks = (first_yd_of_first_week_next_year - first_yd_of_first_week_this_year) / 7;

    WeeksInYear {
        yd_of_first_weekday: first_yd_of_first_week_this_year,
        weeks,
    }
}
