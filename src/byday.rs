use crate::error::IResult;
use crate::mappings;
use crate::util::{is_leap_year, parse_i32, year_len};
use crate::weekday::Weekday;
use chrono::{Datelike, NaiveDate};
use nom::branch::alt;
use nom::combinator::{cut, map, map_res};
use nom::sequence::tuple;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ByDay {
    All(Weekday),
    Nth(Weekday, i32),
}

#[derive(Debug, thiserror::Error)]
#[error("invalid nth {0} for weekday")]
pub struct InvalidNth(i32);

impl ByDay {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        alt((
            map_res(
                tuple((parse_i32, cut(Weekday::parse))),
                |(nth, weekday)| -> Result<Self, InvalidNth> {
                    if !matches!(nth, 1..=53 | -53..=-1) {
                        return Err(InvalidNth(nth));
                    }

                    Ok(Self::Nth(weekday, nth))
                },
            ),
            map(Weekday::parse, Self::All),
        ))(i)
    }

    pub(crate) fn days_in_month(self, year: i32, month1: u32) -> DaysInMonthIterator {
        DaysInMonthIterator::new(self, year, month1)
    }

    pub(crate) fn days_in_year(self, year: i32) -> DaysInYearIterator {
        DaysInYearIterator::new(self, year)
    }
}

impl fmt::Display for ByDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByDay::All(weekday) => weekday.fmt(f),
            ByDay::Nth(weekday, nth) => {
                write!(f, "{}{}", nth, weekday)
            }
        }
    }
}

pub(crate) enum DaysInMonthIterator {
    All {
        month1: u32,
        leap_year: bool,
        base_offset: i32,
        nth: i32,
    },
    Nth {
        yd: Option<i32>,
    },
}

impl DaysInMonthIterator {
    fn new(byday: ByDay, year: i32, month1: u32) -> Self {
        match byday {
            ByDay::All(weekday) => {
                let leap_year = is_leap_year(year);
                let first_month_day = NaiveDate::from_ymd(year, month1, 1);
                let first_month_day_yd = first_month_day.ordinal0() as i32;
                let base_offset = weekday.offset_from(first_month_day.weekday()) as i32;
                let base_offset = first_month_day_yd + base_offset;

                Self::All {
                    month1,
                    leap_year,
                    base_offset,
                    nth: 0,
                }
            }
            ByDay::Nth(weekday, nth) => Self::Nth {
                yd: nth_weekday_in_month(year, month1, weekday, nth),
            },
        }
    }
}

impl Iterator for DaysInMonthIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DaysInMonthIterator::All {
                month1,
                leap_year,
                base_offset,
                nth,
            } => {
                let yd = *base_offset + (*nth * 7);

                let same_month = if *leap_year {
                    mappings::YEARDAY_TO_MONTH_LEAPYEAR
                        .get(yd as usize)
                        .map(|&m| m as u32 == *month1)
                        .unwrap_or_default()
                } else {
                    mappings::YEARDAY_TO_MONTH_NORMAL
                        .get(yd as usize)
                        .map(|&m| m as u32 == *month1)
                        .unwrap_or_default()
                };

                same_month.then(|| {
                    *nth += 1;
                    yd
                })
            }
            DaysInMonthIterator::Nth { yd } => yd.take(),
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

            let last_month_day = NaiveDate::from_ymd_opt(year, month1, last_day)?;

            let nth = -(nth + 1);

            let base_offset = weekday.days_until(last_month_day.weekday()) as i32;
            let offset = base_offset + (nth * 7);

            last_month_day.ordinal0() as i32 - offset
        }
        Ordering::Greater => {
            let first_month_day = NaiveDate::from_ymd_opt(year, month1, 1)?;

            let nth = nth - 1;

            let base_offset = weekday.offset_from(first_month_day.weekday()) as i32;
            let offset = base_offset + (nth * 7);

            first_month_day.ordinal0() as i32 + offset
        }
        Ordering::Equal => return None,
    };

    if yd >= 0 {
        let same_month = if leap_year {
            mappings::YEARDAY_TO_MONTH_LEAPYEAR[yd as usize] as u32 == month1
        } else {
            mappings::YEARDAY_TO_MONTH_NORMAL[yd as usize] as u32 == month1
        };

        same_month.then(|| yd)
    } else {
        None
    }
}

pub(crate) enum DaysInYearIterator {
    All {
        year_len: i32,
        base_offset: i32,
        nth: i32,
    },
    Nth {
        yd: Option<i32>,
    },
}

impl DaysInYearIterator {
    fn new(byday: ByDay, year: i32) -> Self {
        match byday {
            ByDay::All(weekday) => {
                let first_year_day = NaiveDate::from_yo(year, 1);
                let year_len = year_len(year) as i32;
                let base_offset = weekday.offset_from(first_year_day.weekday()) as i32;

                Self::All {
                    year_len,
                    base_offset,
                    nth: 0,
                }
            }
            ByDay::Nth(weekday, nth) => Self::Nth {
                yd: nth_weekday_in_year(year, weekday, nth),
            },
        }
    }
}

impl Iterator for DaysInYearIterator {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            DaysInYearIterator::All {
                year_len,
                base_offset,
                nth,
            } => {
                let yd = *base_offset + (*nth * 7);

                if yd >= 0 && yd < *year_len {
                    *nth += 1;
                    Some(yd)
                } else {
                    None
                }
            }
            DaysInYearIterator::Nth { yd } => yd.take(),
        }
    }
}

fn nth_weekday_in_year(year: i32, weekday: Weekday, nth: i32) -> Option<i32> {
    let year_len = year_len(year);

    let yd = match nth.cmp(&0) {
        Ordering::Less => {
            let last_year_day = NaiveDate::from_yo(year, year_len);
            let nth = -(nth + 1);

            let base_offset = weekday.days_until(last_year_day.weekday()) as i32;
            let offset = base_offset + (nth * 7);

            last_year_day.ordinal0() as i32 - offset
        }
        Ordering::Greater => {
            let first_year_day = NaiveDate::from_yo(year, 1);
            let nth = nth - 1;

            let base_offset = weekday.offset_from(first_year_day.weekday()) as i32;
            let offset = base_offset + (nth * 7);

            first_year_day.ordinal0() as i32 + offset
        }
        Ordering::Equal => return None,
    };

    assert!(yd > 0);

    if yd >= 0 && yd < year_len as i32 {
        Some(yd)
    } else {
        None
    }
}
