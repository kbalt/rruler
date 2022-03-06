use crate::dt_prop::DtStart;
use crate::freq::Frequency;
use crate::mappings;
use crate::recur::{ByWeekday, Recur};
use crate::rrule::RRule;
use crate::util::{is_leap_year, year_len};
use crate::weekday::Weekday;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use std::cmp::Ordering;

#[derive(Debug)]
pub struct RRuleIter {
    // constant
    dt_start: DtStart,
    recur: Recur,
    has_tz: bool,

    dt_start_dt: DateTime<Tz>,

    freq: Frequency,
    interval: u32,
    count: Option<u32>,
    week_start: Weekday,

    // changing with progress
    year: i32,

    days: Vec<i32>,
    days_idx: usize,

    current: DateTime<Tz>,
}

#[derive(Debug, Clone, Copy)]
pub enum RRuleIterYield {
    DateTimeLocal(NaiveDateTime),
    DateTimeTz(DateTime<Tz>),
}

impl RRuleIter {
    pub fn new(rrule: &RRule) -> Self {
        let dt_start = rrule.dt_start.0.to_datetime();

        let mut this = Self {
            dt_start: rrule.dt_start,
            recur: rrule.recur.clone(),
            has_tz: rrule.dt_start.0.tz.is_some(),
            freq: rrule.recur.freq,
            dt_start_dt: dt_start,
            current: rrule.dt_start.0.to_datetime(),
            interval: rrule.recur.interval.unwrap_or(1),
            days: vec![],
            days_idx: 0,
            count: rrule.recur.count,
            week_start: rrule.recur.week_start.unwrap_or(Weekday::Monday),
            year: dt_start.year(),
        };
        this.rebuild_days();
        this
    }

    fn rebuild_days(&mut self) {
        self.days.clear();
        self.days_idx = 0;

        let leap_year = is_leap_year(self.year);
        let year_len = if leap_year { 366 } else { 365 };

        let dt_start_monthday1 = self.dt_start_dt.day();

        if self.recur.freq == Frequency::Yearly {
            // BY MONTH
            if self.recur.by_month.is_empty() {
                if let Some(date) =
                    NaiveDate::from_ymd_opt(self.year, self.dt_start_dt.month(), dt_start_monthday1)
                {
                    self.days.push(date.ordinal0() as i32);
                }
            } else {
                for &month1 in &self.recur.by_month {
                    if !self.recur.by_day.is_empty() {
                        for by_day in &self.recur.by_day {
                            match by_day {
                                ByWeekday::All(_) => todo!(),
                                ByWeekday::Nth(wd, nth) => {
                                    if let Some(yd) =
                                        nth_weekday_in_month(self.year, month1, *wd, *nth)
                                    {
                                        self.days.push(yd);
                                    }
                                }
                            }
                        }
                    } else if let Some(date) =
                        NaiveDate::from_ymd_opt(self.year, month1, dt_start_monthday1)
                    {
                        self.days.push(date.ordinal0() as i32);
                    }
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
            for &by_year_day in &self.recur.by_year_day {
                if by_year_day > 0 && by_year_day < year_len {
                    self.days.push(by_year_day);
                } else if by_year_day < 0 && -by_year_day < year_len {
                    self.days.push(year_len + by_year_day);
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

                    if self.by_day_allows(yearday) {
                        self.days.push(yearday);
                    }
                }
            }

            // BY DAY
            if self.recur.by_month_day.is_empty()
                && self.recur.by_week_no.is_empty()
                && self.recur.by_month.is_empty()
            {
                for by_day in &self.recur.by_day {}
            }
        }

        self.days.sort_unstable();
        self.days.dedup();
    }

    fn by_day_allows(&self, yearday: i32) -> bool {
        assert!(yearday > 0);

        if self.recur.by_day.is_empty() {
            return true;
        }

        let weekday = NaiveDate::from_yo(self.year, yearday as u32).weekday();

        self.recur.by_day.iter().any(|by_day| match by_day {
            ByWeekday::All(wd) => *wd == weekday,
            ByWeekday::Nth(wd, offset) => {
                todo!()
            }
        })
    }

    fn to_yield(&self, datetime: DateTime<Tz>) -> RRuleIterYield {
        if self.has_tz {
            RRuleIterYield::DateTimeTz(datetime)
        } else {
            RRuleIterYield::DateTimeLocal(datetime.naive_local())
        }
    }
}

fn nth_weekday_in_month(year: i32, month1: u32, weekday: Weekday, nth: i32) -> Option<i32> {
    let leap_year = is_leap_year(year);

    let yd = match nth.cmp(&0) {
        Ordering::Less => {
            let month_range = if leap_year {
                &mappings::MONTH_TO_YEARDAYS_LEAPYEAR[month1 as usize - 1]
            } else {
                &mappings::MONTH_TO_YEARDAYS_NORMAL[month1 as usize - 1]
            };

            let last_day = month_range.end - month_range.start;

            let last_month_date = NaiveDate::from_ymd_opt(year, month1, last_day)?;

            let nth = -(nth + 1);

            let base_offset = weekday.days_until(last_month_date.weekday()) as i32;
            let offset = base_offset + (nth * 7);

            last_month_date.ordinal0() as i32 - offset
        }
        Ordering::Greater => {
            let first_month_date = NaiveDate::from_ymd_opt(year, month1, 1)?;
            let first_weekday = first_month_date.weekday();

            let base_offset = weekday.offset_from(first_weekday) as i32;

            let offset = base_offset;

            first_month_date.ordinal0() as i32 + offset
        }
        Ordering::Equal => return None,
    };

    assert!(yd > 0);

    let same_month = if is_leap_year(year) {
        mappings::YEARDAY_TO_MONTH_LEAPYEAR[yd as usize] as u32 == month1
    } else {
        mappings::YEARDAY_TO_MONTH_NORMAL[yd as usize] as u32 == month1
    };

    same_month.then(|| yd)
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

        if self.days.len() == self.days_idx {
            self.year += 1;
            self.rebuild_days();
        }

        let mut yd = self.days[self.days_idx];
        self.days_idx += 1;

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

        let date = NaiveDate::from_ymd(year, month as u32, day as u32);
        let datetime = date.and_hms(0, 0, 0);

        Some(self.to_yield(Tz::UTC.from_local_datetime(&datetime).unwrap()))
    }
}
