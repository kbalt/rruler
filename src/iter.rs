use crate::byday::ByDay;
use crate::dt_prop::local_datetime_with_tz;
use crate::freq::Frequency;
use crate::mappings;
use crate::recur::Recur;
use crate::rrule::RRule;
use crate::util::{is_leap_year, year_len};
use crate::weekday::{days_until, Weekday};
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Timelike};
use chrono_tz::Tz;
use std::cmp::Ordering;

pub struct RRuleIter {
    // recurrence rules
    recur: Recur,

    // DTSTART as DateTime<Tz>
    // If DTSTART is floating the timezone is UTC
    dt_start: DateTime<Tz>,

    // Timezone of DTSTART, None if DTSTART is floating
    dt_start_tz: Option<Tz>,

    interval: u32,
    count: Option<u32>,
    week_start: Weekday,

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

    // vector of year days
    //  -1 = 31.12.YYYY-1
    //   0 = 01.01.YYYY
    //   1 = 02.01.YYYY
    // 366 = 01.01.YYYY+1 (assuming YYYY is not a leap year)
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

            interval: rrule.recur.interval.unwrap_or(1),
            count: rrule.recur.count,
            week_start: rrule.recur.week_start.unwrap_or(Weekday::Monday),

            step,

            hours,
            minutes,
            seconds,

            year: dt_start.year(),
            days_idx: 0,
            hours_idx: 0,
            minutes_idx: 0,
            seconds_idx: 0,

            days: vec![],
        };

        // build days array
        this.rebuild_days();

        // skip all days before DTSTART
        let dt_start_yd = dt_start.ordinal0() as i32;
        let dt_start_hour = dt_start.hour();
        let dt_start_minute = dt_start.minute();
        let dt_start_second = dt_start.second();

        // Find day, hour, minute and second and step to it
        if let Some(days_idx) = this.days.iter().position(|&yd| yd >= dt_start_yd) {
            if let Some(hour_idx) = this.hours.iter().position(|&h| h >= dt_start_hour) {
                if let Some(minutes_idx) = this.minutes.iter().position(|&m| m >= dt_start_minute) {
                    if let Some(seconds_idx) =
                        this.seconds.iter().position(|&s| s >= dt_start_second)
                    {
                        this.days_idx = days_idx;
                        this.hours_idx = hour_idx;
                        this.minutes_idx = minutes_idx;
                        this.seconds_idx = seconds_idx;
                        return this;
                    }
                }
            }
        }

        // == FALLTHROUGH ==

        // No viable day in the year to step to.
        // Skip the year by setting every index to max
        // Then just step.
        // TODO: kinda hacky, this may not work in every case, wrap in loop?
        this.days_idx = this.days.len() - 1;
        this.hours_idx = this.hours.len() - 1;
        this.minutes_idx = this.minutes.len() - 1;
        this.seconds_idx = this.seconds.len() - 1;
        (this.step)(&mut this);

        this
    }

    fn add_days_yearly(&mut self) {
        let leap_year = is_leap_year(self.year);
        let year_len = if leap_year { 366 } else { 365 };

        let dt_start_monthday1 = self.dt_start.day();

        // BY MONTH
        for &month1 in &self.recur.by_month {
            if !self.recur.by_day.is_empty() {
                for by_day in &self.recur.by_day {
                    self.days.extend(by_day.days_in_month(self.year, month1));
                }
            } else if let Some(date) =
                NaiveDate::from_ymd_opt(self.year, month1, dt_start_monthday1)
            {
                self.days.push(date.ordinal0() as i32);
            }
        }

        // BY WEEK NO
        let first_day_of_this_year = NaiveDate::from_yo(self.year, 1);
        let first_day_of_next_year = NaiveDate::from_yo(self.year + 1, 1);

        let wkd_of_first_day_this_year = first_day_of_this_year.weekday();
        let wkd_of_first_day_next_year = first_day_of_next_year.weekday();

        let mut wkst_offset_this_year =
            self.week_start.offset_from(wkd_of_first_day_this_year) as i32;
        let mut wkst_offset_next_year =
            self.week_start.offset_from(wkd_of_first_day_next_year) as i32;

        if wkst_offset_this_year > 4 {
            wkst_offset_this_year -= 7;
        }

        if wkst_offset_next_year > 4 {
            wkst_offset_next_year += year_len - 7;
        } else {
            wkst_offset_next_year += year_len;
        }

        for &week_no1 in &self.recur.by_week_no {
            match week_no1.cmp(&0) {
                Ordering::Greater => {
                    let week_no0 = week_no1 - 1;

                    let first_day_of_week = (week_no0 * 7) + wkst_offset_this_year;

                    for mut yd in first_day_of_week..first_day_of_week + 7 {
                        if yd > year_len {
                            yd -= year_len;
                        }

                        self.days.push(yd as i32);
                    }
                }
                Ordering::Less => {
                    let first_day_of_week = wkst_offset_next_year + (week_no1 * 7);

                    for mut yd in first_day_of_week..first_day_of_week + 7 {
                        if yd > year_len {
                            yd -= year_len;
                        }

                        self.days.push(yd as i32);
                    }
                }
                Ordering::Equal => {
                    unreachable!()
                }
            }
        }

        // BY YEAR DAY
        for &by_year_day1 in &self.recur.by_year_day {
            if by_year_day1 > 0 && by_year_day1 < year_len {
                self.days.push(by_year_day1 - 1);
            } else if by_year_day1 < 0 && -by_year_day1 < year_len {
                self.days.push((year_len + by_year_day1) - 1);
            }
        }

        // BY MONTH DAY
        for &by_month_day in &self.recur.by_month_day {
            for month0 in 0..12 {
                let range = if leap_year {
                    &mappings::MONTH_TO_YEARDAYS_LEAPYEAR[month0]
                } else {
                    &mappings::MONTH_TO_YEARDAYS_NORMAL[month0]
                };

                let max_days = (range.end - range.start) as i32;

                let yearday = if by_month_day > 0 && by_month_day < max_days {
                    range.start as i32 + by_month_day - 1
                } else if by_month_day < 0 && -by_month_day < max_days {
                    range.start as i32 + max_days + by_month_day
                } else {
                    continue;
                };

                if self.by_day_allows(month0 as u32 + 1, yearday) {
                    self.days.push(yearday);
                }
            }
        }

        // BY DAY
        if self.recur.by_month_day.is_empty()
            && self.recur.by_week_no.is_empty()
            && self.recur.by_month.is_empty()
        {
            for by_day in &self.recur.by_day {
                self.days.extend(by_day.days_in_year(self.year));
            }
        }

        // not sure about this
        if self.days.is_empty() {
            if let Some(date) =
                NaiveDate::from_ymd_opt(self.year, self.dt_start.month(), dt_start_monthday1)
            {
                self.days.push(date.ordinal0() as i32);
            } else {
                // oof recursion
                self.year += 1;
                self.add_days_yearly();
            }
        }
    }

    fn add_days_monthly(&mut self) {
        let leap_year = is_leap_year(self.year);
        let dt_start_monthday1 = self.dt_start.day();

        for month1 in months(&self.recur.by_month) {
            if self.recur.by_month_day.is_empty() {
                if self.recur.by_day.is_empty() {
                    if let Some(date) =
                        NaiveDate::from_ymd_opt(self.year, month1, dt_start_monthday1)
                    {
                        self.days.push(date.ordinal0() as i32);
                    }
                } else {
                    for by_day in &self.recur.by_day {
                        self.days.extend(by_day.days_in_month(self.year, month1));
                    }
                }
            } else {
                let days_in_month = mappings::days_in_month(leap_year, month1 - 1)
                    .expect("valid months from 1 to 12") as i32;

                for &by_month_day in &self.recur.by_month_day {
                    let date = if by_month_day > 0 {
                        NaiveDate::from_ymd_opt(self.year, month1, by_month_day as u32)
                    } else if by_month_day < 0 && -by_month_day <= days_in_month {
                        let by_month_day = (-by_month_day) - 1;

                        NaiveDate::from_ymd_opt(
                            self.year,
                            month1,
                            (days_in_month - by_month_day) as u32,
                        )
                    } else {
                        None
                    };

                    if let Some(date) = date {
                        let yd = date.ordinal0() as i32;
                        if self.by_day_allows(month1, yd) {
                            self.days.push(yd);
                        }
                    }
                }
            }
        }
    }

    fn add_days_weekly(&mut self) {
        let leap_year = is_leap_year(self.year);

        let first_day = NaiveDate::from_yo(self.year, 1);
        let first_weekday = first_day.weekday();

        let weekdays = if self.recur.by_day.is_empty() {
            vec![self.dt_start.weekday()]
        } else {
            self.recur
                .by_day
                .iter()
                .filter_map(|byday| {
                    if let ByDay::All(weekday) = byday {
                        Some(weekday.to_chrono())
                    } else {
                        None
                    }
                })
                .collect()
        };

        for weekday in weekdays {
            let offset = days_until(first_weekday as u32, weekday as u32);

            for i in 0.. {
                let yd = offset + (i * 7);

                if let Some(month1) = mappings::yearday_to_month(leap_year, yd) {
                    if self.recur.by_month.is_empty()
                        || self
                            .recur
                            .by_month
                            .iter()
                            .any(|&bymonth1| bymonth1 == month1)
                    {
                        self.days.push(yd as i32);
                    }
                } else {
                    break;
                }
            }
        }
    }

    fn add_days_daily(&mut self) {
        for month1 in months(&self.recur.by_month) {
            let days = days(
                self.year,
                month1,
                &self.recur.by_month_day,
                &self.recur.by_day,
            );

            for yd in days {
                self.days.push(yd as i32);
            }
        }
    }

    fn add_days_hourly_minutely_secondly(&mut self) {
        fn matches_by_year_day(year_len: i32, yd: i32) -> impl Fn(&i32) -> bool {
            move |&by_year_day| {
                if by_year_day > 0 && by_year_day <= year_len {
                    let by_year_day = by_year_day - 1;

                    yd == by_year_day
                } else if by_year_day < 0 && -by_year_day <= year_len {
                    let by_year_day = year_len - (-by_year_day);

                    yd == by_year_day
                } else {
                    false
                }
            }
        }

        let year_len = year_len(self.year) as i32;

        for month1 in months(&self.recur.by_month) {
            let days = days(
                self.year,
                month1,
                &self.recur.by_month_day,
                &self.recur.by_day,
            );

            for yd in days {
                let yd = yd as i32;

                // filter BYYEARDAY
                if !self.recur.by_year_day.is_empty()
                    && !self
                        .recur
                        .by_year_day
                        .iter()
                        .any(matches_by_year_day(year_len, yd))
                {
                    continue;
                }

                self.days.push(yd);
            }
        }
    }

    fn rebuild_days(&mut self) {
        self.days.clear();

        match self.recur.freq {
            Frequency::Secondly => self.add_days_hourly_minutely_secondly(),
            Frequency::Minutely => self.add_days_hourly_minutely_secondly(),
            Frequency::Hourly => self.add_days_hourly_minutely_secondly(),
            Frequency::Daily => self.add_days_daily(),
            Frequency::Weekly => self.add_days_weekly(),
            Frequency::Monthly => self.add_days_monthly(),
            Frequency::Yearly => self.add_days_yearly(),
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

    fn to_yield(&self, datetime: NaiveDateTime) -> RRuleIterYield {
        if let Some(tz) = self.dt_start_tz {
            RRuleIterYield::DateTimeTz(local_datetime_with_tz(datetime, tz))
        } else {
            RRuleIterYield::DateTimeLocal(datetime)
        }
    }
}

impl Iterator for RRuleIter {
    type Item = RRuleIterYield;

    fn next(&mut self) -> Option<Self::Item> {
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

        let (year, leap_year) = if yd < 0 {
            let year = self.year - 1;
            let leap_year = is_leap_year(year);

            if leap_year {
                yd += 366;
            } else {
                yd += 365;
            }

            (year, leap_year)
        } else if yd >= year_len(self.year) as i32 {
            let year = self.year + 1;
            let leap_year = is_leap_year(year);

            if leap_year {
                yd -= 366;
            } else {
                yd -= 365;
            }

            (year, leap_year)
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

        for _ in 0..self.interval {
            (self.step)(self);
        }

        Some(self.to_yield(datetime))
    }
}

fn step_second(this: &mut RRuleIter) {
    this.seconds_idx += 1;

    if this.seconds_idx == this.seconds.len() {
        this.seconds_idx = 0;
        step_minute(this);
    }
}

fn step_minute(this: &mut RRuleIter) {
    this.minutes_idx += 1;

    if this.minutes_idx == this.minutes.len() {
        this.minutes_idx = 0;
        step_hour(this);
    }
}

fn step_hour(this: &mut RRuleIter) {
    this.hours_idx += 1;

    if this.hours_idx == this.hours.len() {
        this.hours_idx = 0;
        step_day(this);
    }
}

fn step_day(this: &mut RRuleIter) {
    this.days_idx += 1;

    if this.days_idx == this.days.len() {
        this.days_idx = 0;

        this.year += 1;
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
    let leap_year = is_leap_year(year);

    let days_in_month =
        mappings::days_in_month(leap_year, month1 - 1).expect("only valid months from 1-12");

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
