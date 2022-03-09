use crate::byday::ByDay;
use crate::dt::Dt;
use crate::error::IResult;
use crate::freq::Frequency;
use crate::util::{display_list, parse_i32, parse_list, parse_u32};
use crate::weekday::Weekday;
use nom::branch::alt;
use nom::character::complete::char;
use nom::bytes::complete::tag;
use nom::combinator::{cut, map, map_res, opt};
use nom::error::context;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use std::fmt;

// TODO custom partial eq which disregards ordering of fields
#[derive(Debug, Clone, PartialEq,Eq, PartialOrd, Ord, Hash)]
pub struct Recur {
    pub freq: Frequency,
    pub until: Option<Dt>,
    pub count: Option<u32>,
    pub interval: Option<u32>,
    pub by_second: Vec<u32>,
    pub by_minute: Vec<u32>,
    pub by_hour: Vec<u32>,
    pub by_day: Vec<ByDay>,
    pub by_month_day: Vec<i32>,
    pub by_year_day: Vec<i32>,
    pub by_week_no: Vec<i32>,
    pub by_month: Vec<u32>,
    pub by_set_pos: Vec<i32>,
    pub week_start: Option<Weekday>,
}

pub enum RecurRulePart {
    Freq(Frequency),
    Until(Dt),
    Count(u32),
    Interval(u32),
    BySecond(Vec<u32>),
    ByMinute(Vec<u32>),
    ByHour(Vec<u32>),
    ByDay(Vec<ByDay>),
    ByMonthDay(Vec<i32>),
    ByYearDay(Vec<i32>),
    ByWeekNo(Vec<i32>),
    ByMonth(Vec<u32>),
    BySetPos(Vec<i32>),
    WeekStart(Weekday),
}

impl RecurRulePart {
    pub(crate) fn parse(i: &str) -> IResult<&str, Self> {
        context(
            "invalid rule part",
            alt((
                map(preceded(tag("FREQ="), cut(Frequency::parse)), Self::Freq),
                map(preceded(tag("UNTIL="), cut(Dt::parse)), Self::Until),
                map(preceded(tag("COUNT="), cut(parse_u32)), Self::Count),
                map(preceded(tag("INTERVAL="), cut(parse_u32)), Self::Interval),
                map(
                    preceded(tag("BYSECOND="), cut(parse_list(parse_u32, ','))),
                    Self::BySecond,
                ),
                map(
                    preceded(tag("BYMINUTE="), cut(parse_list(parse_u32, ','))),
                    Self::ByMinute,
                ),
                map(
                    preceded(tag("BYHOUR="), cut(parse_list(parse_u32, ','))),
                    Self::ByHour,
                ),
                map(
                    preceded(tag("BYDAY="), cut(parse_list(ByDay::parse, ','))),
                    Self::ByDay,
                ),
                map(
                    preceded(tag("BYMONTHDAY="), cut(parse_list(parse_i32, ','))),
                    Self::ByMonthDay,
                ),
                map(
                    preceded(tag("BYYEARDAY="), cut(parse_list(parse_i32, ','))),
                    Self::ByYearDay,
                ),
                map(
                    preceded(tag("BYWEEKNO="), cut(parse_list(parse_i32, ','))),
                    Self::ByWeekNo,
                ),
                map(
                    preceded(tag("BYMONTH="), cut(parse_list(parse_u32, ','))),
                    Self::ByMonth,
                ),
                map(
                    preceded(tag("BYSETPOS="), cut(parse_list(parse_i32, ','))),
                    Self::BySetPos,
                ),
                map(preceded(tag("WKST="), cut(Weekday::parse)), Self::WeekStart),
            )),
        )(i)
    }
}

macro_rules! ensure {
    ($expr:expr, $literal:literal) => {
        if !$expr {
            return Err(RecurParseError::DuplicateProperty($literal));
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum RecurParseError {
    #[error("duplicate property {0}")]
    DuplicateProperty(&'static str),
}

impl Recur {
    pub fn parse(i: &str) -> IResult<&str, Self> {
        map_res(
            many1(terminated(RecurRulePart::parse, opt(char(';')))),
            |items| -> Result<Self, RecurParseError> {
                let mut freq = None;
                let mut until = None;
                let mut count = None;
                let mut interval = None;
                let mut by_second = vec![];
                let mut by_minute = vec![];
                let mut by_hour = vec![];
                let mut by_day = vec![];
                let mut by_month_day = vec![];
                let mut by_year_day = vec![];
                let mut by_week_no = vec![];
                let mut by_month = vec![];
                let mut by_set_pos = vec![];
                let mut week_start = None;

                for item in items {
                    match item {
                        RecurRulePart::Freq(f) => {
                            ensure!(freq.is_none(), "FREQ");
                            freq = Some(f);
                        }
                        RecurRulePart::Until(u) => {
                            ensure!(until.is_none(), "UNTIL");
                            until = Some(u)
                        }
                        RecurRulePart::Count(c) => {
                            ensure!(count.is_none(), "COUNT");
                            count = Some(c);
                        }
                        RecurRulePart::Interval(i) => {
                            ensure!(interval.is_none(), "INTERVAL");
                            interval = Some(i);
                        }
                        RecurRulePart::BySecond(b) => {
                            ensure!(by_second.is_empty(), "BYSECOND");
                            by_second = b;
                        }
                        RecurRulePart::ByMinute(b) => {
                            ensure!(by_minute.is_empty(), "BYMINUTE");
                            by_minute = b;
                        }
                        RecurRulePart::ByHour(b) => {
                            ensure!(by_hour.is_empty(), "BYHOUR");
                            by_hour = b;
                        }
                        RecurRulePart::ByDay(b) => {
                            ensure!(by_day.is_empty(), "BYDAY");
                            by_day = b;
                        }
                        RecurRulePart::ByMonthDay(b) => {
                            ensure!(by_month_day.is_empty(), "BYMONTHDAY");
                            by_month_day = b;
                        }
                        RecurRulePart::ByYearDay(b) => {
                            ensure!(by_year_day.is_empty(), "BYYEARDAY");
                            by_year_day = b;
                        }
                        RecurRulePart::ByWeekNo(b) => {
                            ensure!(by_week_no.is_empty(), "BYWEEKNO");
                            by_week_no = b;
                        }
                        RecurRulePart::ByMonth(b) => {
                            ensure!(by_month.is_empty(), "BYMONTH");
                            by_month = b;
                        }
                        RecurRulePart::BySetPos(b) => {
                            ensure!(by_set_pos.is_empty(), "BYSETPOS");
                            by_set_pos = b;
                        }
                        RecurRulePart::WeekStart(w) => {
                            ensure!(week_start.is_none(), "WKST");
                            week_start = Some(w);
                        }
                    }
                }

                let this = Self {
                    // TODO error handling :)
                    freq: freq.unwrap(),
                    until,
                    count,
                    interval,
                    by_second,
                    by_minute,
                    by_hour,
                    by_day,
                    by_month_day,
                    by_year_day,
                    by_week_no,
                    by_month,
                    by_set_pos,
                    week_start,
                };

                Ok(this)
            },
        )(i)
    }

    pub(crate) fn sort_and_dedup(&mut self) {
        self.by_second.sort_unstable();
        self.by_minute.sort_unstable();
        self.by_hour.sort_unstable();
        self.by_day.sort_unstable();
        self.by_month_day.sort_unstable();
        self.by_year_day.sort_unstable();
        self.by_week_no.sort_unstable();
        self.by_month.sort_unstable();
        self.by_set_pos.sort_unstable();

        self.by_second.dedup();
        self.by_minute.dedup();
        self.by_hour.dedup();
        self.by_day.dedup();
        self.by_month_day.dedup();
        self.by_year_day.dedup();
        self.by_week_no.dedup();
        self.by_month.dedup();
        self.by_set_pos.dedup();
    }
}

impl fmt::Display for Recur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FREQ={}", self.freq)?;

        if let Some(interval) = self.interval {
            write!(f, ";INTERVAL={}", interval)?;
        }

        display_list(f, "BYMONTH", &self.by_month)?;
        display_list(f, "BYWEEKNO", &self.by_week_no)?;
        display_list(f, "BYYEARDAY", &self.by_year_day)?;
        display_list(f, "BYMONTHDAY", &self.by_month_day)?;
        display_list(f, "BYDAY", &self.by_day)?;
        display_list(f, "BYHOUR", &self.by_hour)?;
        display_list(f, "BYMINUTE", &self.by_minute)?;
        display_list(f, "BYSECOND", &self.by_second)?;
        display_list(f, "BYSETPOS", &self.by_set_pos)?;

        if let Some(week_start) = self.week_start {
            write!(f, ";WKST={}", week_start)?;
        }

        if let Some(count) = self.count {
            write!(f, ";COUNT={}", count)?;
        }

        if let Some(until) = self.until {
            write!(f, ";UNTIL={}", until)?;
        }

        Ok(())
    }
}
