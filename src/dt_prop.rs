use crate::dt::Dt;
use crate::error::{IResult, ParseError};
use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::char;
use nom::combinator::{cut, map, map_res};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{preceded, tuple};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum DtParam {
    ValueDate,
    ValueDateTime,
    Tz(Tz),
}

impl DtParam {
    fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map(
                preceded(
                    tag_no_case("TZID="),
                    cut(map_res(take_while1(|c| c != ';' && c != ':'), |tz| {
                        Tz::from_str(tz).map_err(ParseError::from_tz_parse)
                    })),
                ),
                Self::Tz,
            ),
            map(tag_no_case("VALUE=DATE"), |_| Self::ValueDate),
            map(tag_no_case("VALUE=DATETIME"), |_| Self::ValueDateTime),
        ))(i)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DtPropertyParseError {
    #[error("duplicate parameter {0}")]
    DuplicateParam(&'static str),
    #[error("failed to convert floating datetime")]
    InvalidFloatingTime,
    #[error("value parameter specified {0} (default=DATETIME) but got {1}")]
    InvalidValueParam(&'static str, &'static str),
}

#[derive(Debug, Clone, Copy)]
pub struct DtProperty {
    pub dt: Dt,
    pub tz: Option<Tz>,
}

impl DtProperty {
    pub(crate) fn to_datetime(self) -> DateTime<Tz> {
        let tz = self.tz.unwrap_or(Tz::UTC);

        match self.dt {
            Dt::Date(date) => tz.from_local_date(&date).unwrap().and_hms(0, 0, 0),
            Dt::DateTimeLocal(datetime) => tz.from_local_datetime(&datetime).unwrap(),
            Dt::DateTimeUtc(datetime) => datetime.with_timezone(&tz),
        }
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        context(
            "invalid dt property",
            map_res(
                tuple((
                    many0(preceded(char(';'), cut(DtParam::parse))),
                    preceded(char(':'), Dt::parse),
                )),
                |(params, dt)| -> Result<Self, DtPropertyParseError> {
                    let mut is_datetime = None;
                    let mut tz = None;

                    for param in params {
                        match param {
                            DtParam::ValueDate => {
                                if is_datetime.is_some() {
                                    return Err(DtPropertyParseError::DuplicateParam("VALUE"));
                                }

                                is_datetime = Some(false);
                            }
                            DtParam::ValueDateTime => {
                                if is_datetime.is_some() {
                                    return Err(DtPropertyParseError::DuplicateParam("VALUE"));
                                }

                                is_datetime = Some(true);
                            }
                            DtParam::Tz(t) => {
                                if tz.is_some() {
                                    return Err(DtPropertyParseError::DuplicateParam("TZID"));
                                }

                                tz = Some(t);
                            }
                        }
                    }

                    let is_datetime = is_datetime.unwrap_or(true);

                    match dt {
                        Dt::Date(_) => {
                            if is_datetime {
                                return Err(DtPropertyParseError::InvalidValueParam(
                                    "DATETIME", "DATE",
                                ));
                            }

                            // TODO is tz relevant here?
                        }
                        Dt::DateTimeLocal(_) => {
                            if !is_datetime {
                                return Err(DtPropertyParseError::InvalidValueParam(
                                    "DATE", "DATETIME",
                                ));
                            }
                        }
                        Dt::DateTimeUtc(_) => {
                            if !is_datetime {
                                return Err(DtPropertyParseError::InvalidValueParam(
                                    "DATE", "DATETIME",
                                ));
                            }
                        }
                    }

                    Ok(Self { dt, tz })
                },
            ),
        )(i)
    }
}

impl fmt::Display for DtProperty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(tz) = self.tz {
            write!(f, ";TZID={}", tz)?;
        }

        if self.dt.is_date() {
            write!(f, ";VALUE=DATE")?;
        }

        write!(f, ":{}", self.dt)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DtStart(pub DtProperty);

impl DtStart {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(preceded(tag("DTSTART"), DtProperty::parse), Self)(i)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RDate(pub DtProperty);

impl RDate {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(preceded(tag("RDATE"), DtProperty::parse), Self)(i)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ExDate(pub DtProperty);

impl ExDate {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map(preceded(tag("EXDATE"), DtProperty::parse), Self)(i)
    }
}
