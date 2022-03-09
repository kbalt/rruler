use super::{months, RRuleIter};
use crate::mappings::{self};
use chrono::{Datelike, NaiveDate};

impl RRuleIter {
    pub(super) fn rebuild_days_monthly(&mut self) {
        // TODO: loop limit
        while {
            self.add_days_monthly();

            if self.days.is_empty() {
                self.year += 1;
                true
            } else {
                false
            }
        } {}
    }

    fn add_days_monthly(&mut self) {
        let dt_start_monthday1 = self.dt_start.day();

        let dt_start_month = if self.year == self.dt_start.year() {
            Some(self.dt_start.month())
        } else {
            None
        };

        let dt_start_yd = if self.year == self.dt_start.year() {
            Some(self.dt_start.ordinal0() as i32)
        } else {
            None
        };

        for month1 in months(&self.recur.by_month) {
            if let Some(dt_start_month) = dt_start_month {
                if month1 < dt_start_month {
                    continue;
                }
            }

            let skip = self.nth_unit != 0;
            self.nth_unit += 1;
            self.nth_unit %= self.interval as u64;
            if skip {
                continue;
            }

            if self.recur.by_month_day.is_empty() {
                if self.recur.by_day.is_empty() {
                    if let Some(date) =
                        NaiveDate::from_ymd_opt(self.year, month1, dt_start_monthday1)
                    {
                        self.days.push(date.ordinal0() as i32);
                    }
                } else {
                    for by_day in &self.recur.by_day {
                        for yd in by_day.days_in_month(self.year, month1) {
                            if let Some(dt_start_yd) = dt_start_yd {
                                if yd < dt_start_yd {
                                    continue;
                                }
                            }

                            self.days.push(yd);
                        }
                    }
                }
            } else {
                let days_in_month = mappings::days_in_month(self.year, month1 - 1)
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

                        if let Some(dt_start_yd) = dt_start_yd {
                            if yd < dt_start_yd {
                                continue;
                            }
                        }

                        if self.by_day_allows(month1, yd) {
                            self.days.push(yd);
                        }
                    }
                }
            }
        }
    }
}
