use crate::error::IResult;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use nom::bytes::complete::take;
use nom::character::complete::char;
use nom::combinator::{map, map_res, opt};
use nom::sequence::{preceded, tuple};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub enum Dt {
    Date(NaiveDate),
    DateTimeLocal(NaiveDateTime),
    DateTimeUtc(DateTime<Utc>),
}

impl Dt {
    pub fn is_date(&self) -> bool {
        matches!(self, Self::Date(_))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DtParseError {
    #[error("failed to parse integer for component {0}")]
    NotANumber(&'static str),
    #[error("given date was invalid")]
    InvalidDate,
    #[error("given time was invalid")]
    InvalidTime,
}

impl Dt {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        map_res(
            tuple((
                tuple((take(4usize), take(2usize), take(2usize))),
                opt(preceded(
                    char('T'),
                    tuple((
                        take(2usize),
                        take(2usize),
                        take(2usize),
                        map(opt(char('Z')), |x| x.is_some()),
                    )),
                )),
            )),
            |((year, month, day), time)| -> Result<Self, DtParseError> {
                let year = i32::from_str(year).map_err(|_| DtParseError::NotANumber("year"))?;
                let month = u32::from_str(month).map_err(|_| DtParseError::NotANumber("month"))?;
                let day = u32::from_str(day).map_err(|_| DtParseError::NotANumber("day"))?;

                let naive_date =
                    NaiveDate::from_ymd_opt(year, month, day).ok_or(DtParseError::InvalidDate)?;

                if let Some((hour, minute, second, is_utc)) = time {
                    let hour = u32::from_str(hour).map_err(|_| DtParseError::NotANumber("hour"))?;
                    let minute =
                        u32::from_str(minute).map_err(|_| DtParseError::NotANumber("minute"))?;
                    let second =
                        u32::from_str(second).map_err(|_| DtParseError::NotANumber("second"))?;

                    let naive_time = NaiveTime::from_hms_opt(hour, minute, second)
                        .ok_or(DtParseError::InvalidTime)?;
                    let naive_datetime = NaiveDateTime::new(naive_date, naive_time);

                    if is_utc {
                        Ok(Self::DateTimeUtc(
                            Utc.from_local_datetime(&naive_datetime)
                                .single()
                                .ok_or(DtParseError::InvalidTime)?,
                        ))
                    } else {
                        Ok(Self::DateTimeLocal(naive_datetime))
                    }
                } else {
                    Ok(Self::Date(naive_date))
                }
            },
        )(i)
    }
}

impl fmt::Display for Dt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Dt::Date(date) => date.format("%Y%m%d").fmt(f),
            Dt::DateTimeLocal(datetime) => datetime.format("%Y%m%dT%H%M%S").fmt(f),
            Dt::DateTimeUtc(datetime) => datetime.format("%Y%m%dT%H%M%SZ").fmt(f),
        }
    }
}
