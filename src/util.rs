use crate::error::IResult;
use crate::mappings::days_in_month;
use chrono::{Datelike, NaiveDate};
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::combinator::{map_parser, map_res, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::terminated;
use std::cmp::Ordering;
use std::fmt;
use std::str::FromStr;

pub(crate) fn display_list<I, D>(f: &mut fmt::Formatter<'_>, name: &str, list: I) -> fmt::Result
where
    I: IntoIterator<Item = D>,
    D: fmt::Display,
{
    let mut iter = list.into_iter();

    if let Some(value) = iter.next() {
        write!(f, ";{}={}", name, value)?;

        for value in iter {
            write!(f, ",{}", value)?;
        }
    }

    Ok(())
}

pub(crate) fn parse_list<F, T>(f: F, del: char) -> impl Fn(&str) -> IResult<&str, Vec<T>>
where
    F: Fn(&str) -> IResult<&str, T> + Copy,
{
    move |i| {
        many1(terminated(
            map_parser(take_while1(|c: char| !c.is_whitespace() && c != del), f),
            opt(char(del)),
        ))(i)
    }
}

pub(crate) fn parse_i32(i: &str) -> IResult<&str, i32> {
    context(
        "expected i32",
        map_res(
            take_while1(|c| matches!(c, '0'..='9' | '-' | '+')),
            FromStr::from_str,
        ),
    )(i)
}

pub(crate) fn parse_u32(i: &str) -> IResult<&str, u32> {
    context(
        "expected u32",
        map_res(
            take_while1(|c| matches!(c, '0'..='9' | '+')),
            FromStr::from_str,
        ),
    )(i)
}

pub(crate) fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

// Get the amount of days in the year
pub(crate) fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

// Convert a yearday to the corresponding weekday
pub(crate) fn yd_to_weekday(mut year: i32, mut yd: i32) -> chrono::Weekday {
    let year_len = days_in_year(year) as i32;

    // check if its in the previous year
    if yd < 0 {
        year -= 1;
        yd += year_len;
    } else if yd >= year_len {
        // or in the next year
        year += 1;
        yd -= days_in_year(year) as i32;
    }

    NaiveDate::from_yo(year, (yd + 1) as u32).weekday()
}

pub(crate) fn by_month_day_to_monthday1(year: i32, month1: u32, by_month_day1: i32) -> Option<u32> {
    let days_in_month = days_in_month(year, month1 - 1)? as i32;

    match by_month_day1.cmp(&0) {
        Ordering::Greater if by_month_day1 <= days_in_month => Some(by_month_day1 as u32),
        Ordering::Less if -by_month_day1 <= days_in_month => {
            Some((days_in_month + by_month_day1 + 1) as u32)
        }
        _ => None,
    }
}

pub(crate) fn by_year_day_to_yd(year: i32, by_year_day1: i32) -> Option<i32> {
    let year_len = days_in_year(year) as i32;

    match by_year_day1.cmp(&0) {
        Ordering::Greater if by_year_day1 <= year_len => Some(by_year_day1 - 1),
        Ordering::Less if -by_year_day1 <= year_len => Some(year_len + by_year_day1),
        _ => None,
    }
}

pub(crate) fn yd_to_week_no0(first_yd_of_year: i32, weeks: i32, yd: i32) -> u32 {
    if first_yd_of_year > yd {
        assert!(yd - first_yd_of_year < 7);
        12
    } else {
        let days_since_first_yd = yd - first_yd_of_year;
        let week = days_since_first_yd / 7;

        if week >= weeks {
            u32::try_from(week - weeks).unwrap()
        } else {
            u32::try_from(week).unwrap()
        }
    }
}

pub(crate) fn by_week_no_to_week_no0(weeks: i32, by_week_no: i32) -> Option<u32> {
    match by_week_no.cmp(&0) {
        Ordering::Greater if by_week_no <= weeks => Some((by_week_no - 1) as u32),
        Ordering::Less if -by_week_no <= weeks => Some((weeks + by_week_no) as u32),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn by_month_day_to_monthday() {
        assert_eq!(by_month_day_to_monthday1(2020, 1, 1), Some(1));
        assert_eq!(by_month_day_to_monthday1(2020, 1, 2), Some(2));
        assert_eq!(by_month_day_to_monthday1(2020, 1, 31), Some(31));
        assert_eq!(by_month_day_to_monthday1(2020, 1, 32), None);

        assert_eq!(by_month_day_to_monthday1(2020, 1, -1), Some(31));
        assert_eq!(by_month_day_to_monthday1(2020, 1, -2), Some(30));
        assert_eq!(by_month_day_to_monthday1(2020, 1, -31), Some(1));
        assert_eq!(by_month_day_to_monthday1(2020, 1, -32), None);
    }

    #[test]
    fn by_year_day_to_yearday() {
        assert_eq!(by_year_day_to_yd(2020, 1), Some(0));
        assert_eq!(by_year_day_to_yd(2020, 2), Some(1));
        assert_eq!(by_year_day_to_yd(2020, 366), Some(365));
        assert_eq!(by_year_day_to_yd(2020, 367), None);

        assert_eq!(by_year_day_to_yd(2020, -1), Some(365));
        assert_eq!(by_year_day_to_yd(2020, -2), Some(364));
        assert_eq!(by_year_day_to_yd(2020, -366), Some(0));
        assert_eq!(by_year_day_to_yd(2020, -367), None);
    }

    #[test]
    fn week_no0_of_yearday() {
        assert_eq!(yd_to_week_no0(0, 53, 0), 0);
        assert_eq!(yd_to_week_no0(0, 53, 1), 0);
        assert_eq!(yd_to_week_no0(0, 53, 2), 0);
        assert_eq!(yd_to_week_no0(0, 53, 3), 0);
        assert_eq!(yd_to_week_no0(0, 53, 4), 0);
        assert_eq!(yd_to_week_no0(0, 53, 5), 0);
        assert_eq!(yd_to_week_no0(0, 53, 6), 0);
        assert_eq!(yd_to_week_no0(0, 53, 7), 1);
        assert_eq!(yd_to_week_no0(0, 53, 8), 1);
        assert_eq!(yd_to_week_no0(0, 53, 9), 1);
        assert_eq!(yd_to_week_no0(0, 53, 10), 1);
        assert_eq!(yd_to_week_no0(0, 53, 11), 1);
        assert_eq!(yd_to_week_no0(0, 53, 12), 1);
        assert_eq!(yd_to_week_no0(0, 53, 13), 1);
        assert_eq!(yd_to_week_no0(0, 53, 14), 2);
        assert_eq!(yd_to_week_no0(0, 53, 15), 2);

        assert_eq!(yd_to_week_no0(-2, 53, 0), 0);
        assert_eq!(yd_to_week_no0(-2, 53, 1), 0);
        assert_eq!(yd_to_week_no0(-2, 53, 2), 0);
        assert_eq!(yd_to_week_no0(-2, 53, 3), 0);
        assert_eq!(yd_to_week_no0(-2, 53, 4), 0);
        assert_eq!(yd_to_week_no0(-2, 53, 5), 1);
        assert_eq!(yd_to_week_no0(-2, 53, 6), 1);
    }
}
