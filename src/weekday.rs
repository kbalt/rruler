use crate::error::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::context;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weekday {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Saturday = 5,
    Sunday = 6,
}

impl PartialEq<chrono::Weekday> for Weekday {
    fn eq(&self, other: &chrono::Weekday) -> bool {
        *self as u32 == *other as u32
    }
}

impl Weekday {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        context(
            "invalid weekday value",
            alt((
                map(tag("MO"), |_| Self::Monday),
                map(tag("TU"), |_| Self::Tuesday),
                map(tag("WE"), |_| Self::Wednesday),
                map(tag("TH"), |_| Self::Thursday),
                map(tag("FR"), |_| Self::Friday),
                map(tag("SA"), |_| Self::Saturday),
                map(tag("SU"), |_| Self::Sunday),
            )),
        )(i)
    }

    pub(crate) fn days_until(self, until: chrono::Weekday) -> u32 {
        days_until(self as u32, until as u32)
    }

    pub(crate) fn offset_from(self, from: chrono::Weekday) -> u32 {
        days_until(from as u32, self as u32)
    }
}

impl fmt::Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Monday => f.write_str("MO"),
            Self::Tuesday => f.write_str("TU"),
            Self::Wednesday => f.write_str("WE"),
            Self::Thursday => f.write_str("TH"),
            Self::Friday => f.write_str("FR"),
            Self::Saturday => f.write_str("SA"),
            Self::Sunday => f.write_str("SU"),
        }
    }
}

fn days_until(weekday: u32, until: u32) -> u32 {
    if weekday <= until {
        until - weekday
    } else {
        7 - (weekday - until)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::Weekday as CWD;

    #[test]
    fn days_until() {
        assert_eq!(CWD::Mon as u32, 0);
        assert_eq!(Weekday::Monday as u32, 0);

        assert_eq!(Weekday::Monday.days_until(CWD::Mon), 0);
        assert_eq!(Weekday::Monday.days_until(CWD::Tue), 1);
        assert_eq!(Weekday::Monday.days_until(CWD::Wed), 2);
        assert_eq!(Weekday::Monday.days_until(CWD::Thu), 3);
        assert_eq!(Weekday::Monday.days_until(CWD::Fri), 4);
        assert_eq!(Weekday::Monday.days_until(CWD::Sat), 5);
        assert_eq!(Weekday::Monday.days_until(CWD::Sun), 6);

        assert_eq!(Weekday::Tuesday.days_until(CWD::Mon), 6);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Tue), 0);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Wed), 1);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Thu), 2);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Fri), 3);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Sat), 4);
        assert_eq!(Weekday::Tuesday.days_until(CWD::Sun), 5);

        assert_eq!(Weekday::Wednesday.days_until(CWD::Mon), 5);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Tue), 6);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Wed), 0);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Thu), 1);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Fri), 2);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Sat), 3);
        assert_eq!(Weekday::Wednesday.days_until(CWD::Sun), 4);

        assert_eq!(Weekday::Thursday.days_until(CWD::Mon), 4);
        assert_eq!(Weekday::Thursday.days_until(CWD::Tue), 5);
        assert_eq!(Weekday::Thursday.days_until(CWD::Wed), 6);
        assert_eq!(Weekday::Thursday.days_until(CWD::Thu), 0);
        assert_eq!(Weekday::Thursday.days_until(CWD::Fri), 1);
        assert_eq!(Weekday::Thursday.days_until(CWD::Sat), 2);
        assert_eq!(Weekday::Thursday.days_until(CWD::Sun), 3);

        assert_eq!(Weekday::Friday.days_until(CWD::Mon), 3);
        assert_eq!(Weekday::Friday.days_until(CWD::Tue), 4);
        assert_eq!(Weekday::Friday.days_until(CWD::Wed), 5);
        assert_eq!(Weekday::Friday.days_until(CWD::Thu), 6);
        assert_eq!(Weekday::Friday.days_until(CWD::Fri), 0);
        assert_eq!(Weekday::Friday.days_until(CWD::Sat), 1);
        assert_eq!(Weekday::Friday.days_until(CWD::Sun), 2);

        assert_eq!(Weekday::Saturday.days_until(CWD::Mon), 2);
        assert_eq!(Weekday::Saturday.days_until(CWD::Tue), 3);
        assert_eq!(Weekday::Saturday.days_until(CWD::Wed), 4);
        assert_eq!(Weekday::Saturday.days_until(CWD::Thu), 5);
        assert_eq!(Weekday::Saturday.days_until(CWD::Fri), 6);
        assert_eq!(Weekday::Saturday.days_until(CWD::Sat), 0);
        assert_eq!(Weekday::Saturday.days_until(CWD::Sun), 1);

        assert_eq!(Weekday::Sunday.days_until(CWD::Mon), 1);
        assert_eq!(Weekday::Sunday.days_until(CWD::Tue), 2);
        assert_eq!(Weekday::Sunday.days_until(CWD::Wed), 3);
        assert_eq!(Weekday::Sunday.days_until(CWD::Thu), 4);
        assert_eq!(Weekday::Sunday.days_until(CWD::Fri), 5);
        assert_eq!(Weekday::Sunday.days_until(CWD::Sat), 6);
        assert_eq!(Weekday::Sunday.days_until(CWD::Sun), 0);
    }
}
