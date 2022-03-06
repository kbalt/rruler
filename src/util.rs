use crate::error::IResult;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::combinator::{map_parser, map_res, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::terminated;
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
            map_parser(take_while1(|c| c != del), f),
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

pub(crate) fn year_len(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}
