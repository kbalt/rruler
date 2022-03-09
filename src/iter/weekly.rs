use super::{weeks_in_year, RRuleIter, WeeksInYear};
use crate::byday::ByDay;
use crate::mappings::yearday_to_month1;
use crate::weekday::days_until;
use chrono::Datelike;

impl RRuleIter {
    pub(super) fn rebuild_days_weekly(&mut self) {
        // TODO: loop limit
        while {
            self.add_days_weekly();

            if self.days.is_empty() {
                self.year += 1;
                true
            } else {
                false
            }
        } {}
    }

    fn add_days_weekly(&mut self) {
        let (week_start, weekdays) = if self.recur.by_day.is_empty() {
            (self.dt_start.weekday(), vec![self.dt_start.weekday()])
        } else {
            (
                self.week_start.to_chrono(),
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
                    .collect(),
            )
        };

        let WeeksInYear {
            yd_of_first_weekday,
            weeks,
        } = weeks_in_year(self.year, week_start);

        let mut week_days = Vec::with_capacity(weekdays.len());

        let dt_start_yd = if self.year == self.dt_start.year() {
            Some(self.dt_start.ordinal0() as i32)
        } else {
            None
        };

        for week_no in 0..weeks {
            let first_day_in_this_week = yd_of_first_weekday + (week_no * 7);
            let last_day_in_this_week = first_day_in_this_week + 7;

            // skip weeks until DTSTART
            if let Some(dt_start_yd) = dt_start_yd {
                // skip this week if dtstart is part of the next week
                if dt_start_yd >= last_day_in_this_week {
                    continue;
                }
            }

            let skip = self.nth_unit % self.interval as u64 != 0;
            self.nth_unit += 1;
            if skip {
                continue;
            }

            for &weekday in &weekdays {
                let offset = days_until(week_start as u32, weekday as u32) as i32;
                let yd = first_day_in_this_week + offset;

                let month1 = yearday_to_month1(self.year, yd);

                if self.recur.by_month.is_empty()
                    || self
                        .recur
                        .by_month
                        .iter()
                        .any(|&bymonth1| bymonth1 == month1)
                {
                    // skip any days in the week before DTSTART
                    if let Some(dt_start_yd) = dt_start_yd {
                        if dt_start_yd > yd {
                            continue;
                        }
                    }

                    week_days.push(yd as i32);
                }
            }

            // TODO BYSETPOS handling in week_days here

            self.days.append(&mut week_days);
        }

        // not sure about this
        if self.days.is_empty() {
            // oof recursion
            self.year += 1;
            self.add_days_weekly();
        }
    }
}
