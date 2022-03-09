use super::{days, months, RRuleIter};
use crate::util::by_year_day_to_yd;
use chrono::Datelike;

impl RRuleIter {
    pub(super) fn rebuild_days_hms_ly(&mut self) {
        // TODO: loop limit
        while {
            self.add_days_hms_ly();

            if self.days.is_empty() {
                self.year += 1;
                true
            } else {
                false
            }
        } {}
    }

    fn add_days_hms_ly(&mut self) {
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

                // filter BYYEARDAY
                if !self.recur.by_year_day.is_empty()
                    && !self
                        .recur
                        .by_year_day
                        .iter()
                        .filter_map(|&by_year_day1| by_year_day_to_yd(self.year, by_year_day1))
                        .any(|by_year_day| by_year_day == yd)
                {
                    continue;
                }

                self.days.push(yd);
            }
        }
    }
}
