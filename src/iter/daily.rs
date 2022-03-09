use super::{days, months, RRuleIter};
use chrono::Datelike;

impl RRuleIter {
    pub(crate) fn rebuild_days_daily(&mut self) {
        // TODO: loop limit
        while {
            self.add_days_daily();

            if self.days.is_empty() {
                self.year += 1;
                true
            } else {
                false
            }
        } {}
    }

    fn add_days_daily(&mut self) {
        let dt_start_yd = if self.year == self.dt_start.year() {
            Some(self.dt_start.ordinal0() as i32)
        } else {
            None
        };

        for month1 in months(&self.recur.by_month) {
            let days = days(
                self.year,
                month1,
                &self.recur.by_month_day,
                &self.recur.by_day,
            );

            for yd in days {
                let yd = yd as i32;

                // skip any days in the week before DTSTART
                if let Some(dt_start_yd) = dt_start_yd {
                    if dt_start_yd > yd {
                        continue;
                    }
                }

                let skip = self.nth_unit % self.interval as u64 != 0;
                self.nth_unit += 1;
                if skip {
                    continue;
                }

                self.days.push(yd);
            }
        }
    }
}
