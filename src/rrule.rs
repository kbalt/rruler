use crate::dt::Dt;
use crate::dt_prop::DtStart;
use crate::error::IResult;
use crate::freq::Frequency;
use crate::recur::{ByWeekday, Recur};
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map;
use nom::sequence::{preceded, terminated, tuple};

#[derive(Debug)]
pub struct RRule {
    pub(crate) dt_start: DtStart,
    pub(crate) recur: Recur,
}

#[derive(Debug, thiserror::Error)]
pub enum RRuleVerifyError {
    #[error("invalid INTERVAL {0}")]
    InvalidInterval(u32),
    #[error("DTSTART's and UNTIL's value type does not match")]
    UntilInvalidValueType,
    #[error("DTSTART is local but UNTIL is not")]
    UntilNotLocal,
    #[error("DTSTART specifies a timezone but UNTIL is not UTC")]
    UntilNotUtc,
    #[error("strict: BYSECOND, BYMINUTE and BYHOUR not allowed if DTSTART's VALUE type is DATE")]
    StrictByXNotAllowedInDateType,
    #[error("BYDAY offset specified with FREQ YEARLY and BYWEEKNO")]
    ByDayOffSetNotAllowedWithFreqYearlyAndByWeekNo,

    #[error("invalid {0} value {1}")]
    InvalidValue(&'static str, i32),
    #[error("[0] not allowed in FREQ {1}")]
    NotAllowedInFreq(&'static str, Frequency),
}

impl RRule {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                terminated(DtStart::parse, take_while1(|c| matches!(c, '\r' | '\n'))),
                preceded(tag("RRULE:"), Recur::parse),
            )),
            |(dt_start, recur)| Self { dt_start, recur },
        )(i)
    }

    pub fn verify(&self, strict: bool) -> Result<(), RRuleVerifyError> {
        if let Some(interval) = self.recur.interval {
            if interval == 0 {
                return Err(RRuleVerifyError::InvalidInterval(0));
            }
        }

        if let Some(until) = self.recur.until {
            //  The value of the UNTIL rule part MUST have the same value type as the "DTSTART" property.
            if until.is_date() != self.dt_start.0.dt.is_date() {
                return Err(RRuleVerifyError::UntilInvalidValueType);
            }

            let dt_start_is_local = matches!(self.dt_start.0.dt, Dt::DateTimeLocal(_));
            let dt_start_has_tzid = self.dt_start.0.tz.is_none();
            let dt_start_is_utc = matches!(self.dt_start.0.dt, Dt::DateTimeUtc(_));

            let until_is_local = matches!(until, Dt::DateTimeLocal(_));
            let until_is_utc = matches!(until, Dt::DateTimeUtc(_));

            // Furthermore, if the "DTSTART" property is specified as a date with local time,
            // then the UNTIL rule part MUST also be specified as a date with local time.
            if dt_start_is_local && !dt_start_has_tzid && !until_is_local {
                return Err(RRuleVerifyError::UntilNotLocal);
            }

            // If the "DTSTART" property is specified as a date with UTC time or a date with local time and time zone
            // reference, then the UNTIL rule part MUST be specified as a date with UTC time.
            if (dt_start_is_utc || (dt_start_is_local && dt_start_has_tzid)) && !until_is_utc {
                return Err(RRuleVerifyError::UntilNotUtc);
            }
        }

        // The BYSECOND, BYMINUTE and BYHOUR rule parts MUST NOT be specified
        // when the associated "DTSTART" property has a DATE value type.
        if strict
            && self.dt_start.0.dt.is_date()
            && (!self.recur.by_second.is_empty()
                || self.recur.by_minute.is_empty()
                || self.recur.by_hour.is_empty())
        {
            return Err(RRuleVerifyError::StrictByXNotAllowedInDateType);
        }

        let mut by_day_offset_specified = false;

        for by_day in &self.recur.by_day {
            if let ByWeekday::Nth(_, nth) = by_day {
                if !matches!(nth, 1..=53 | -53..=-1) {
                    return Err(RRuleVerifyError::InvalidValue("BYDAY nth", *nth));
                }

                by_day_offset_specified = true;
            }
        }

        // The BYDAY rule part MUST NOT be specified with a numeric value
        // when the FREQ rule part is not set to MONTHLY or YEARLY.
        if by_day_offset_specified
            && !matches!(self.recur.freq, Frequency::Monthly | Frequency::Yearly)
        {
            return Err(RRuleVerifyError::NotAllowedInFreq(
                "BYDAY offset",
                self.recur.freq,
            ));
        }

        // Furthermore, the BYDAY rule part MUST NOT be specified with a numeric value
        // with the FREQ rule part set to YEARLY when the BYWEEKNO rule part is specified.
        if by_day_offset_specified
            && matches!(self.recur.freq, Frequency::Yearly)
            && !self.recur.by_week_no.is_empty()
        {
            return Err(RRuleVerifyError::ByDayOffSetNotAllowedWithFreqYearlyAndByWeekNo);
        }

        // BYMONTHDAY
        for offset in &self.recur.by_month_day {
            if !matches!(offset, 1..=31 | -31..=-1) {
                return Err(RRuleVerifyError::InvalidValue("BYMONTHDAY", *offset));
            }
        }

        if matches!(self.recur.freq, Frequency::Weekly) && !self.recur.by_month_day.is_empty() {
            return Err(RRuleVerifyError::NotAllowedInFreq(
                "BYMONTHDAY",
                Frequency::Weekly,
            ));
        }

        // BYYEARDAY
        for offset in &self.recur.by_year_day {
            if !matches!(offset, 1..=366 | -366..=-1) {
                return Err(RRuleVerifyError::InvalidValue("BYYEARDAY", *offset));
            }
        }

        // The BYYEARDAY rule part MUST NOT be specified when the FREQ rule part is set to DAILY, WEEKLY, or MONTHLY.
        if matches!(
            self.recur.freq,
            Frequency::Daily | Frequency::Weekly | Frequency::Monthly
        ) && !self.recur.by_year_day.is_empty()
        {
            return Err(RRuleVerifyError::NotAllowedInFreq(
                "BYYEARDAY",
                self.recur.freq,
            ));
        }

        // BYWEEKNO
        for week_no in &self.recur.by_week_no {
            if !matches!(week_no, 1..=53 | -53..=-1) {
                return Err(RRuleVerifyError::InvalidValue("BYWEEKNO", *week_no));
            }
        }

        // This rule part MUST NOT be used when the FREQ rule part is set to anything other than YEARLY.
        if !matches!(self.recur.freq, Frequency::Yearly) && !self.recur.by_week_no.is_empty() {
            return Err(RRuleVerifyError::NotAllowedInFreq(
                "BYWEEKNO",
                self.recur.freq,
            ));
        }

        for month in &self.recur.by_month {
            if !matches!(month, 1..=12) {
                return Err(RRuleVerifyError::InvalidValue("BYMONTH", *month as i32));
            }
        }

        // BYSETPOS
        for offset in &self.recur.by_set_pos {
            if !matches!(offset, 1..=366 | -366..=-1) {
                return Err(RRuleVerifyError::InvalidValue("BYSETPOS", *offset));
            }
        }

        Ok(())
    }
}
