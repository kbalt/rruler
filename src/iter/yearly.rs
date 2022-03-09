use super::{months, weeks_in_year, RRuleIter, WeeksInYear};
use crate::byday::ByDay;
use crate::mappings::{yearday_to_month1, yearday_to_monthday1};
use crate::util::{
    by_month_day_to_monthday1, by_week_no_to_week_no0, by_year_day_to_yd, yd_to_week_no0,
    yd_to_weekday,
};
use chrono::naive::MAX_DATE;
use chrono::{Datelike, NaiveDate};

impl RRuleIter {
    pub(super) fn rebuild_days_yearly(&mut self) {
        while {
            self.add_days_yearly();

            if self.days.is_empty() {
                self.year += self.interval as i32;
                if self.year >= MAX_DATE.year() - 1 {
                    self.done = true;
                    false
                } else {
                    true
                }
            } else {
                false
            }
        } {}
    }

    fn add_days_yearly(&mut self) {
        let dt_start_monthday1 = self.dt_start.day();

        let dt_start_yd = if self.year == self.dt_start.year() {
            Some(self.dt_start.ordinal0() as i32)
        } else {
            None
        };

        let WeeksInYear {
            yd_of_first_weekday,
            weeks,
        } = weeks_in_year(self.year, self.week_start.to_chrono());

        // BY MONTH
        for &month1 in &self.recur.by_month {
            if self.recur.by_day.is_empty() {
                if let Some(date) = NaiveDate::from_ymd_opt(self.year, month1, dt_start_monthday1) {
                    let yd = date.ordinal0() as i32;

                    if let Some(dt_start_yd) = dt_start_yd {
                        if yd < dt_start_yd {
                            continue;
                        }
                    }

                    if !self.recur.by_week_no.is_empty() {
                        let yd_week_no0 = yd_to_week_no0(yd_of_first_weekday, weeks, yd);

                        if !self
                            .recur
                            .by_week_no
                            .iter()
                            .filter_map(|&by_week_no| by_week_no_to_week_no0(weeks, by_week_no))
                            .any(|week_no0| week_no0 == yd_week_no0)
                        {
                            continue;
                        }
                    }

                    if !self.recur.by_year_day.is_empty()
                        && !self
                            .recur
                            .by_year_day
                            .iter()
                            .filter_map(|&by_year_day1| by_year_day_to_yd(self.year, by_year_day1))
                            .any(|by_year_day0| yd == by_year_day0)
                    {
                        continue;
                    }

                    if !self.recur.by_month_day.is_empty() {
                        let yd_month_day1 = yearday_to_monthday1(self.year, yd);

                        if !self
                            .recur
                            .by_month_day
                            .iter()
                            .filter_map(|&by_month_day1| {
                                by_month_day_to_monthday1(self.year, month1, by_month_day1)
                            })
                            .any(|month_day1| month_day1 == yd_month_day1)
                        {
                            continue;
                        }
                    }

                    if self.by_day_allows(month1, yd) {
                        self.days.push(yd);
                    }
                }
            } else {
                for by_day in &self.recur.by_day {
                    for yd in by_day.days_in_month(self.year, month1) {
                        if let Some(dt_start_yd) = dt_start_yd {
                            if yd < dt_start_yd {
                                continue;
                            }
                        }

                        if !self.recur.by_week_no.is_empty() {
                            let yd_week_no0 = yd_to_week_no0(yd_of_first_weekday, weeks, yd);

                            if !self
                                .recur
                                .by_week_no
                                .iter()
                                .filter_map(|&by_week_no| by_week_no_to_week_no0(weeks, by_week_no))
                                .any(|week_no0| week_no0 == yd_week_no0)
                            {
                                continue;
                            }
                        }

                        if !self.recur.by_year_day.is_empty()
                            && !self
                                .recur
                                .by_year_day
                                .iter()
                                .filter_map(|&by_year_day1| {
                                    by_year_day_to_yd(self.year, by_year_day1)
                                })
                                .any(|by_year_day0| yd == by_year_day0)
                        {
                            continue;
                        }

                        if !self.recur.by_month_day.is_empty() {
                            let yd_month_day1 = yearday_to_monthday1(self.year, yd);

                            if !self
                                .recur
                                .by_month_day
                                .iter()
                                .filter_map(|&by_month_day1| {
                                    by_month_day_to_monthday1(self.year, month1, by_month_day1)
                                })
                                .any(|month_day1| month_day1 == yd_month_day1)
                            {
                                continue;
                            }
                        }

                        self.days.push(yd);
                    }
                }
            }
        }

        // BY WEEK NO
        for &by_week_no1 in &self.recur.by_week_no {
            let week_no0 = if let Some(week_no0) = by_week_no_to_week_no0(weeks, by_week_no1) {
                week_no0 as i32
            } else {
                continue;
            };

            let first_day_in_this_week = yd_of_first_weekday + (week_no0 * 7);
            let last_day_in_this_week = first_day_in_this_week + 7;

            for yd in first_day_in_this_week..last_day_in_this_week {
                if let Some(dt_start_yd) = dt_start_yd {
                    if yd < dt_start_yd {
                        continue;
                    }
                }

                if !self.recur.by_month.is_empty() {
                    let m = yearday_to_month1(self.year, yd);
                    if !self.recur.by_month.contains(&m) {
                        continue;
                    }
                }

                if !self.recur.by_year_day.is_empty()
                    && !self
                        .recur
                        .by_year_day
                        .iter()
                        .filter_map(|&by_year_day1| by_year_day_to_yd(self.year, by_year_day1))
                        .any(|by_year_day0| yd == by_year_day0)
                {
                    continue;
                }

                if !self.recur.by_month_day.is_empty() {
                    let month1 = yearday_to_month1(self.year, yd);
                    let yd_month_day1 = yearday_to_monthday1(self.year, yd);

                    if !self
                        .recur
                        .by_month_day
                        .iter()
                        .filter_map(|&by_month_day1| {
                            by_month_day_to_monthday1(self.year, month1, by_month_day1)
                        })
                        .any(|month_day1| month_day1 == yd_month_day1)
                    {
                        continue;
                    }
                }

                if self.recur.by_day.is_empty() {
                    self.days.push(yd);
                } else {
                    let weekday = yd_to_weekday(self.year, yd);

                    for &by_day in &self.recur.by_day {
                        if let ByDay::All(wd) = by_day {
                            if weekday == wd.to_chrono() {
                                self.days.push(yd);
                                break;
                            }
                        }
                    }
                }
            }
        }

        if self.recur.by_month.is_empty()
            && self.recur.by_week_no.is_empty()
            && self.recur.by_month_day.is_empty()
        {
            // BY YEAR DAY
            for &by_year_day1 in &self.recur.by_year_day {
                let yd = if let Some(yd) = by_year_day_to_yd(self.year, by_year_day1) {
                    yd
                } else {
                    continue;
                };

                if let Some(dt_start_yd) = dt_start_yd {
                    if yd < dt_start_yd {
                        continue;
                    }
                }

                self.days.push(yd);
            }
        }

        // TODO; also check if BYYEARDAY is empty?
        if self.recur.by_month.is_empty() && self.recur.by_week_no.is_empty() {
            // BY MONTH DAY
            for &by_month_day1 in &self.recur.by_month_day {
                for month1 in months(&self.recur.by_month) {
                    let md = if let Some(md) =
                        by_month_day_to_monthday1(self.year, month1, by_month_day1)
                    {
                        md
                    } else {
                        continue;
                    };

                    let yd = if let Some(date) = NaiveDate::from_ymd_opt(self.year, month1, md) {
                        date.ordinal0() as i32
                    } else {
                        continue;
                    };

                    if let Some(dt_start_yd) = dt_start_yd {
                        if yd < dt_start_yd {
                            continue;
                        }
                    }

                    if self.by_day_allows(month1 as u32 + 1, yd) {
                        self.days.push(yd);
                    }
                }
            }
        }

        if self.recur.by_month.is_empty()
            && self.recur.by_week_no.is_empty()
            && self.recur.by_year_day.is_empty()
            && self.recur.by_month_day.is_empty()
        {
            if self.recur.by_day.is_empty() {
                if let Some(date) =
                    NaiveDate::from_ymd_opt(self.year, self.dt_start.month(), dt_start_monthday1)
                {
                    let yd = date.ordinal0() as i32;

                    if let Some(dt_start_yd) = dt_start_yd {
                        if yd >= dt_start_yd {
                            self.days.push(yd);
                        }
                    } else {
                        self.days.push(yd);
                    }
                }
            } else {
                for by_day in &self.recur.by_day {
                    for yd in by_day.days_in_year(self.year) {
                        if let Some(dt_start_yd) = dt_start_yd {
                            if yd < dt_start_yd {
                                continue;
                            }
                        }

                        self.days.push(yd);
                    }
                }
            }
        }
    }
}
