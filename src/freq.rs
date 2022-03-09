use crate::error::IResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::error::context;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq,Eq, PartialOrd, Ord, Hash)]
pub enum Frequency {
    Secondly,
    Minutely,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

impl Frequency {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        context(
            "invalid FREQ value",
            alt((
                map(tag("SECONDLY"), |_| Self::Secondly),
                map(tag("MINUTELY"), |_| Self::Minutely),
                map(tag("HOURLY"), |_| Self::Hourly),
                map(tag("DAILY"), |_| Self::Daily),
                map(tag("WEEKLY"), |_| Self::Weekly),
                map(tag("MONTHLY"), |_| Self::Monthly),
                map(tag("YEARLY"), |_| Self::Yearly),
            )),
        )(i)
    }
}

impl fmt::Display for Frequency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Secondly => f.write_str("SECONDLY"),
            Self::Minutely => f.write_str("MINUTELY"),
            Self::Hourly => f.write_str("HOURLY"),
            Self::Daily => f.write_str("DAILY"),
            Self::Weekly => f.write_str("WEEKLY"),
            Self::Monthly => f.write_str("MONTHLY"),
            Self::Yearly => f.write_str("YEARLY"),
        }
    }
}
