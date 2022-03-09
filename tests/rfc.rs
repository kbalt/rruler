use chrono::{DateTime, TimeZone};
use chrono_tz::Tz;
use pretty_assertions::assert_eq;
use std::str::FromStr;

fn test(input: &str) -> Vec<DateTime<Tz>> {
    let rrule = rruler::rrule::RRule::from_str(input).unwrap();
    rruler::iter::RRuleIter::new(&rrule)
        .map(|y| match y {
            rruler::iter::RRuleIterYield::DateTimeLocal(_) => unreachable!(),
            rruler::iter::RRuleIterYield::DateTimeTz(dt) => dt,
        })
        .take(1000)
        .collect()
}

fn ymd_hms(year: i32, mon: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Tz> {
    chrono_tz::America::New_York
        .ymd(year, mon, day)
        .and_hms(hour, min, sec)
}

#[test]
fn daily_for_10_occurrences() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;COUNT=10",
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 5, 9, 0, 0),
            ymd_hms(1997, 9, 6, 9, 0, 0),
            ymd_hms(1997, 9, 7, 9, 0, 0),
            ymd_hms(1997, 9, 8, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 10, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
        ]
    );
}

#[test]
fn daily_until_dec_24_1997() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;UNTIL=19971224T000000Z;COUNT=113", // TODO remove COUNT=112 when UNTIL is implemented
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 5, 9, 0, 0),
            ymd_hms(1997, 9, 6, 9, 0, 0),
            ymd_hms(1997, 9, 7, 9, 0, 0),
            ymd_hms(1997, 9, 8, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 10, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 12, 9, 0, 0),
            ymd_hms(1997, 9, 13, 9, 0, 0),
            ymd_hms(1997, 9, 14, 9, 0, 0),
            ymd_hms(1997, 9, 15, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 17, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
            ymd_hms(1997, 9, 19, 9, 0, 0),
            ymd_hms(1997, 9, 20, 9, 0, 0),
            ymd_hms(1997, 9, 21, 9, 0, 0),
            ymd_hms(1997, 9, 22, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 24, 9, 0, 0),
            ymd_hms(1997, 9, 25, 9, 0, 0),
            ymd_hms(1997, 9, 26, 9, 0, 0),
            ymd_hms(1997, 9, 27, 9, 0, 0),
            ymd_hms(1997, 9, 28, 9, 0, 0),
            ymd_hms(1997, 9, 29, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
            ymd_hms(1997, 10, 4, 9, 0, 0),
            ymd_hms(1997, 10, 5, 9, 0, 0),
            ymd_hms(1997, 10, 6, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
            ymd_hms(1997, 10, 8, 9, 0, 0),
            ymd_hms(1997, 10, 9, 9, 0, 0),
            ymd_hms(1997, 10, 10, 9, 0, 0),
            ymd_hms(1997, 10, 11, 9, 0, 0),
            ymd_hms(1997, 10, 12, 9, 0, 0),
            ymd_hms(1997, 10, 13, 9, 0, 0),
            ymd_hms(1997, 10, 14, 9, 0, 0),
            ymd_hms(1997, 10, 15, 9, 0, 0),
            ymd_hms(1997, 10, 16, 9, 0, 0),
            ymd_hms(1997, 10, 17, 9, 0, 0),
            ymd_hms(1997, 10, 18, 9, 0, 0),
            ymd_hms(1997, 10, 19, 9, 0, 0),
            ymd_hms(1997, 10, 20, 9, 0, 0),
            ymd_hms(1997, 10, 21, 9, 0, 0),
            ymd_hms(1997, 10, 22, 9, 0, 0),
            ymd_hms(1997, 10, 23, 9, 0, 0),
            ymd_hms(1997, 10, 24, 9, 0, 0),
            ymd_hms(1997, 10, 25, 9, 0, 0),
            ymd_hms(1997, 10, 26, 9, 0, 0),
            ymd_hms(1997, 10, 27, 9, 0, 0),
            ymd_hms(1997, 10, 28, 9, 0, 0),
            ymd_hms(1997, 10, 29, 9, 0, 0),
            ymd_hms(1997, 10, 30, 9, 0, 0),
            ymd_hms(1997, 10, 31, 9, 0, 0),
            ymd_hms(1997, 11, 1, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
            ymd_hms(1997, 11, 3, 9, 0, 0),
            ymd_hms(1997, 11, 4, 9, 0, 0),
            ymd_hms(1997, 11, 5, 9, 0, 0),
            ymd_hms(1997, 11, 6, 9, 0, 0),
            ymd_hms(1997, 11, 7, 9, 0, 0),
            ymd_hms(1997, 11, 8, 9, 0, 0),
            ymd_hms(1997, 11, 9, 9, 0, 0),
            ymd_hms(1997, 11, 10, 9, 0, 0),
            ymd_hms(1997, 11, 11, 9, 0, 0),
            ymd_hms(1997, 11, 12, 9, 0, 0),
            ymd_hms(1997, 11, 13, 9, 0, 0),
            ymd_hms(1997, 11, 14, 9, 0, 0),
            ymd_hms(1997, 11, 15, 9, 0, 0),
            ymd_hms(1997, 11, 16, 9, 0, 0),
            ymd_hms(1997, 11, 17, 9, 0, 0),
            ymd_hms(1997, 11, 18, 9, 0, 0),
            ymd_hms(1997, 11, 19, 9, 0, 0),
            ymd_hms(1997, 11, 20, 9, 0, 0),
            ymd_hms(1997, 11, 21, 9, 0, 0),
            ymd_hms(1997, 11, 22, 9, 0, 0),
            ymd_hms(1997, 11, 23, 9, 0, 0),
            ymd_hms(1997, 11, 24, 9, 0, 0),
            ymd_hms(1997, 11, 25, 9, 0, 0),
            ymd_hms(1997, 11, 26, 9, 0, 0),
            ymd_hms(1997, 11, 27, 9, 0, 0),
            ymd_hms(1997, 11, 28, 9, 0, 0),
            ymd_hms(1997, 11, 29, 9, 0, 0),
            ymd_hms(1997, 11, 30, 9, 0, 0),
            ymd_hms(1997, 12, 1, 9, 0, 0),
            ymd_hms(1997, 12, 2, 9, 0, 0),
            ymd_hms(1997, 12, 3, 9, 0, 0),
            ymd_hms(1997, 12, 4, 9, 0, 0),
            ymd_hms(1997, 12, 5, 9, 0, 0),
            ymd_hms(1997, 12, 6, 9, 0, 0),
            ymd_hms(1997, 12, 7, 9, 0, 0),
            ymd_hms(1997, 12, 8, 9, 0, 0),
            ymd_hms(1997, 12, 9, 9, 0, 0),
            ymd_hms(1997, 12, 10, 9, 0, 0),
            ymd_hms(1997, 12, 11, 9, 0, 0),
            ymd_hms(1997, 12, 12, 9, 0, 0),
            ymd_hms(1997, 12, 13, 9, 0, 0),
            ymd_hms(1997, 12, 14, 9, 0, 0),
            ymd_hms(1997, 12, 15, 9, 0, 0),
            ymd_hms(1997, 12, 16, 9, 0, 0),
            ymd_hms(1997, 12, 17, 9, 0, 0),
            ymd_hms(1997, 12, 18, 9, 0, 0),
            ymd_hms(1997, 12, 19, 9, 0, 0),
            ymd_hms(1997, 12, 20, 9, 0, 0),
            ymd_hms(1997, 12, 21, 9, 0, 0),
            ymd_hms(1997, 12, 22, 9, 0, 0),
            ymd_hms(1997, 12, 23, 9, 0, 0),
        ]
    );
}

#[test]
fn every_other_day() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;INTERVAL=2;COUNT=10",
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 6, 9, 0, 0),
            ymd_hms(1997, 9, 8, 9, 0, 0),
            ymd_hms(1997, 9, 10, 9, 0, 0),
            ymd_hms(1997, 9, 12, 9, 0, 0),
            ymd_hms(1997, 9, 14, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
            ymd_hms(1997, 9, 20, 9, 0, 0),
        ]
    );
}

#[test]
fn every_10_days_5_occurrences() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;INTERVAL=10;COUNT=5",
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 12, 9, 0, 0),
            ymd_hms(1997, 9, 22, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 10, 12, 9, 0, 0),
        ]
    );
}

#[test]
fn every_day_in_january_for_3_years_yearly() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19980101T090000\n\
        RRULE:FREQ=YEARLY;UNTIL=20000131T140000Z;BYMONTH=1;BYDAY=SU,MO,TU,WE,TH,FR,SA;COUNT=93", // TODO remove COUNT when UNTIL is implemented
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 1, 3, 9, 0, 0),
            ymd_hms(1998, 1, 4, 9, 0, 0),
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
            ymd_hms(1998, 1, 9, 9, 0, 0),
            ymd_hms(1998, 1, 10, 9, 0, 0),
            ymd_hms(1998, 1, 11, 9, 0, 0),
            ymd_hms(1998, 1, 12, 9, 0, 0),
            ymd_hms(1998, 1, 13, 9, 0, 0),
            ymd_hms(1998, 1, 14, 9, 0, 0),
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 16, 9, 0, 0),
            ymd_hms(1998, 1, 17, 9, 0, 0),
            ymd_hms(1998, 1, 18, 9, 0, 0),
            ymd_hms(1998, 1, 19, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 1, 21, 9, 0, 0),
            ymd_hms(1998, 1, 22, 9, 0, 0),
            ymd_hms(1998, 1, 23, 9, 0, 0),
            ymd_hms(1998, 1, 24, 9, 0, 0),
            ymd_hms(1998, 1, 25, 9, 0, 0),
            ymd_hms(1998, 1, 26, 9, 0, 0),
            ymd_hms(1998, 1, 27, 9, 0, 0),
            ymd_hms(1998, 1, 28, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 1, 30, 9, 0, 0),
            ymd_hms(1998, 1, 31, 9, 0, 0),
            ymd_hms(1999, 1, 1, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(1999, 1, 5, 9, 0, 0),
            ymd_hms(1999, 1, 6, 9, 0, 0),
            ymd_hms(1999, 1, 7, 9, 0, 0),
            ymd_hms(1999, 1, 8, 9, 0, 0),
            ymd_hms(1999, 1, 9, 9, 0, 0),
            ymd_hms(1999, 1, 10, 9, 0, 0),
            ymd_hms(1999, 1, 11, 9, 0, 0),
            ymd_hms(1999, 1, 12, 9, 0, 0),
            ymd_hms(1999, 1, 13, 9, 0, 0),
            ymd_hms(1999, 1, 14, 9, 0, 0),
            ymd_hms(1999, 1, 15, 9, 0, 0),
            ymd_hms(1999, 1, 16, 9, 0, 0),
            ymd_hms(1999, 1, 17, 9, 0, 0),
            ymd_hms(1999, 1, 18, 9, 0, 0),
            ymd_hms(1999, 1, 19, 9, 0, 0),
            ymd_hms(1999, 1, 20, 9, 0, 0),
            ymd_hms(1999, 1, 21, 9, 0, 0),
            ymd_hms(1999, 1, 22, 9, 0, 0),
            ymd_hms(1999, 1, 23, 9, 0, 0),
            ymd_hms(1999, 1, 24, 9, 0, 0),
            ymd_hms(1999, 1, 25, 9, 0, 0),
            ymd_hms(1999, 1, 26, 9, 0, 0),
            ymd_hms(1999, 1, 27, 9, 0, 0),
            ymd_hms(1999, 1, 28, 9, 0, 0),
            ymd_hms(1999, 1, 29, 9, 0, 0),
            ymd_hms(1999, 1, 30, 9, 0, 0),
            ymd_hms(1999, 1, 31, 9, 0, 0),
            ymd_hms(2000, 1, 1, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 5, 9, 0, 0),
            ymd_hms(2000, 1, 6, 9, 0, 0),
            ymd_hms(2000, 1, 7, 9, 0, 0),
            ymd_hms(2000, 1, 8, 9, 0, 0),
            ymd_hms(2000, 1, 9, 9, 0, 0),
            ymd_hms(2000, 1, 10, 9, 0, 0),
            ymd_hms(2000, 1, 11, 9, 0, 0),
            ymd_hms(2000, 1, 12, 9, 0, 0),
            ymd_hms(2000, 1, 13, 9, 0, 0),
            ymd_hms(2000, 1, 14, 9, 0, 0),
            ymd_hms(2000, 1, 15, 9, 0, 0),
            ymd_hms(2000, 1, 16, 9, 0, 0),
            ymd_hms(2000, 1, 17, 9, 0, 0),
            ymd_hms(2000, 1, 18, 9, 0, 0),
            ymd_hms(2000, 1, 19, 9, 0, 0),
            ymd_hms(2000, 1, 20, 9, 0, 0),
            ymd_hms(2000, 1, 21, 9, 0, 0),
            ymd_hms(2000, 1, 22, 9, 0, 0),
            ymd_hms(2000, 1, 23, 9, 0, 0),
            ymd_hms(2000, 1, 24, 9, 0, 0),
            ymd_hms(2000, 1, 25, 9, 0, 0),
            ymd_hms(2000, 1, 26, 9, 0, 0),
            ymd_hms(2000, 1, 27, 9, 0, 0),
            ymd_hms(2000, 1, 28, 9, 0, 0),
            ymd_hms(2000, 1, 29, 9, 0, 0),
            ymd_hms(2000, 1, 30, 9, 0, 0),
            ymd_hms(2000, 1, 31, 9, 0, 0),
        ]
    );
}

#[test]
fn every_day_in_january_for_3_years_daily() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19980101T090000\n\
        RRULE:FREQ=DAILY;UNTIL=20000131T140000Z;BYMONTH=1;COUNT=93", // TODO remove COUNT when UNTIL is implemented
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 1, 3, 9, 0, 0),
            ymd_hms(1998, 1, 4, 9, 0, 0),
            ymd_hms(1998, 1, 5, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 7, 9, 0, 0),
            ymd_hms(1998, 1, 8, 9, 0, 0),
            ymd_hms(1998, 1, 9, 9, 0, 0),
            ymd_hms(1998, 1, 10, 9, 0, 0),
            ymd_hms(1998, 1, 11, 9, 0, 0),
            ymd_hms(1998, 1, 12, 9, 0, 0),
            ymd_hms(1998, 1, 13, 9, 0, 0),
            ymd_hms(1998, 1, 14, 9, 0, 0),
            ymd_hms(1998, 1, 15, 9, 0, 0),
            ymd_hms(1998, 1, 16, 9, 0, 0),
            ymd_hms(1998, 1, 17, 9, 0, 0),
            ymd_hms(1998, 1, 18, 9, 0, 0),
            ymd_hms(1998, 1, 19, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 1, 21, 9, 0, 0),
            ymd_hms(1998, 1, 22, 9, 0, 0),
            ymd_hms(1998, 1, 23, 9, 0, 0),
            ymd_hms(1998, 1, 24, 9, 0, 0),
            ymd_hms(1998, 1, 25, 9, 0, 0),
            ymd_hms(1998, 1, 26, 9, 0, 0),
            ymd_hms(1998, 1, 27, 9, 0, 0),
            ymd_hms(1998, 1, 28, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 1, 30, 9, 0, 0),
            ymd_hms(1998, 1, 31, 9, 0, 0),
            ymd_hms(1999, 1, 1, 9, 0, 0),
            ymd_hms(1999, 1, 2, 9, 0, 0),
            ymd_hms(1999, 1, 3, 9, 0, 0),
            ymd_hms(1999, 1, 4, 9, 0, 0),
            ymd_hms(1999, 1, 5, 9, 0, 0),
            ymd_hms(1999, 1, 6, 9, 0, 0),
            ymd_hms(1999, 1, 7, 9, 0, 0),
            ymd_hms(1999, 1, 8, 9, 0, 0),
            ymd_hms(1999, 1, 9, 9, 0, 0),
            ymd_hms(1999, 1, 10, 9, 0, 0),
            ymd_hms(1999, 1, 11, 9, 0, 0),
            ymd_hms(1999, 1, 12, 9, 0, 0),
            ymd_hms(1999, 1, 13, 9, 0, 0),
            ymd_hms(1999, 1, 14, 9, 0, 0),
            ymd_hms(1999, 1, 15, 9, 0, 0),
            ymd_hms(1999, 1, 16, 9, 0, 0),
            ymd_hms(1999, 1, 17, 9, 0, 0),
            ymd_hms(1999, 1, 18, 9, 0, 0),
            ymd_hms(1999, 1, 19, 9, 0, 0),
            ymd_hms(1999, 1, 20, 9, 0, 0),
            ymd_hms(1999, 1, 21, 9, 0, 0),
            ymd_hms(1999, 1, 22, 9, 0, 0),
            ymd_hms(1999, 1, 23, 9, 0, 0),
            ymd_hms(1999, 1, 24, 9, 0, 0),
            ymd_hms(1999, 1, 25, 9, 0, 0),
            ymd_hms(1999, 1, 26, 9, 0, 0),
            ymd_hms(1999, 1, 27, 9, 0, 0),
            ymd_hms(1999, 1, 28, 9, 0, 0),
            ymd_hms(1999, 1, 29, 9, 0, 0),
            ymd_hms(1999, 1, 30, 9, 0, 0),
            ymd_hms(1999, 1, 31, 9, 0, 0),
            ymd_hms(2000, 1, 1, 9, 0, 0),
            ymd_hms(2000, 1, 2, 9, 0, 0),
            ymd_hms(2000, 1, 3, 9, 0, 0),
            ymd_hms(2000, 1, 4, 9, 0, 0),
            ymd_hms(2000, 1, 5, 9, 0, 0),
            ymd_hms(2000, 1, 6, 9, 0, 0),
            ymd_hms(2000, 1, 7, 9, 0, 0),
            ymd_hms(2000, 1, 8, 9, 0, 0),
            ymd_hms(2000, 1, 9, 9, 0, 0),
            ymd_hms(2000, 1, 10, 9, 0, 0),
            ymd_hms(2000, 1, 11, 9, 0, 0),
            ymd_hms(2000, 1, 12, 9, 0, 0),
            ymd_hms(2000, 1, 13, 9, 0, 0),
            ymd_hms(2000, 1, 14, 9, 0, 0),
            ymd_hms(2000, 1, 15, 9, 0, 0),
            ymd_hms(2000, 1, 16, 9, 0, 0),
            ymd_hms(2000, 1, 17, 9, 0, 0),
            ymd_hms(2000, 1, 18, 9, 0, 0),
            ymd_hms(2000, 1, 19, 9, 0, 0),
            ymd_hms(2000, 1, 20, 9, 0, 0),
            ymd_hms(2000, 1, 21, 9, 0, 0),
            ymd_hms(2000, 1, 22, 9, 0, 0),
            ymd_hms(2000, 1, 23, 9, 0, 0),
            ymd_hms(2000, 1, 24, 9, 0, 0),
            ymd_hms(2000, 1, 25, 9, 0, 0),
            ymd_hms(2000, 1, 26, 9, 0, 0),
            ymd_hms(2000, 1, 27, 9, 0, 0),
            ymd_hms(2000, 1, 28, 9, 0, 0),
            ymd_hms(2000, 1, 29, 9, 0, 0),
            ymd_hms(2000, 1, 30, 9, 0, 0),
            ymd_hms(2000, 1, 31, 9, 0, 0),
        ]
    );
}

#[test]
fn weekly_for_10_occurrences() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;COUNT=10",
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
            ymd_hms(1997, 10, 14, 9, 0, 0),
            ymd_hms(1997, 10, 21, 9, 0, 0),
            ymd_hms(1997, 10, 28, 9, 0, 0),
            ymd_hms(1997, 11, 4, 9, 0, 0),
        ]
    );
}

#[test]
fn weekly_until_december_24_1997() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;UNTIL=19971224T000000Z;COUNT=17", // TODO remove COUNT=17 once `UNTIL=` is implemented
    );

    assert_eq!(
        dts,
        vec![
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
            ymd_hms(1997, 10, 14, 9, 0, 0),
            ymd_hms(1997, 10, 21, 9, 0, 0),
            ymd_hms(1997, 10, 28, 9, 0, 0),
            ymd_hms(1997, 11, 4, 9, 0, 0),
            ymd_hms(1997, 11, 11, 9, 0, 0),
            ymd_hms(1997, 11, 18, 9, 0, 0),
            ymd_hms(1997, 11, 25, 9, 0, 0),
            ymd_hms(1997, 12, 2, 9, 0, 0),
            ymd_hms(1997, 12, 9, 9, 0, 0),
            ymd_hms(1997, 12, 16, 9, 0, 0),
            ymd_hms(1997, 12, 23, 9, 0, 0),
        ]
    );
}

#[test]
fn every_other_week() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;WKST=SU;COUNT=12",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 14, 9, 0, 0),
            ymd_hms(1997, 10, 28, 9, 0, 0),
            ymd_hms(1997, 11, 11, 9, 0, 0),
            ymd_hms(1997, 11, 25, 9, 0, 0),
            ymd_hms(1997, 12, 9, 9, 0, 0),
            ymd_hms(1997, 12, 23, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 2, 3, 9, 0, 0),
        ]
    );
}

#[test]
fn weeky_on_tuesday_and_thursday_for_five_weeks_1() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;UNTIL=19971007T000000Z;WKST=SU;BYDAY=TU,TH;COUNT=10", // TODO remove `COUNT=10` when UNTIL= is implemented
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 25, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
        ]
    );
}

/// Weekly on Tuesday and Thursday for five weeks:
#[test]
fn weeky_on_tuesday_and_thursday_for_five_weeks_2() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;COUNT=10;WKST=SU;BYDAY=TU,TH",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 25, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
        ]
    );
}

/// Every other week on Monday, Wednesday, and Friday until December 24, 1997, starting on Monday, September 1, 1997:
#[test]
fn every_other_week_on_monday_wednesday_and_friday_until_dec_24_1997_starting_sept_1_1997() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970901T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;UNTIL=19971224T000000Z;WKST=SU;BYDAY=MO,WE,FR;COUNT=25", // TODO remove `count=25` when UNTIL is implemented
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 1, 9, 0, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
            ymd_hms(1997, 9, 5, 9, 0, 0),
            ymd_hms(1997, 9, 15, 9, 0, 0),
            ymd_hms(1997, 9, 17, 9, 0, 0),
            ymd_hms(1997, 9, 19, 9, 0, 0),
            ymd_hms(1997, 9, 29, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
            ymd_hms(1997, 10, 13, 9, 0, 0),
            ymd_hms(1997, 10, 15, 9, 0, 0),
            ymd_hms(1997, 10, 17, 9, 0, 0),
            ymd_hms(1997, 10, 27, 9, 0, 0),
            ymd_hms(1997, 10, 29, 9, 0, 0),
            ymd_hms(1997, 10, 31, 9, 0, 0),
            ymd_hms(1997, 11, 10, 9, 0, 0),
            ymd_hms(1997, 11, 12, 9, 0, 0),
            ymd_hms(1997, 11, 14, 9, 0, 0),
            ymd_hms(1997, 11, 24, 9, 0, 0),
            ymd_hms(1997, 11, 26, 9, 0, 0),
            ymd_hms(1997, 11, 28, 9, 0, 0),
            ymd_hms(1997, 12, 8, 9, 0, 0),
            ymd_hms(1997, 12, 10, 9, 0, 0),
            ymd_hms(1997, 12, 12, 9, 0, 0),
            ymd_hms(1997, 12, 22, 9, 0, 0),
        ]
    );
}

#[test]
fn every_other_week_on_tuesday_and_thursday_for_8_occurrences() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=8;WKST=SU;BYDAY=TU,TH",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 18, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 10, 14, 9, 0, 0),
            ymd_hms(1997, 10, 16, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_first_friday_10_occurrences() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970905T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYDAY=1FR",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 5, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
            ymd_hms(1997, 11, 7, 9, 0, 0),
            ymd_hms(1997, 12, 5, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 2, 6, 9, 0, 0),
            ymd_hms(1998, 3, 6, 9, 0, 0),
            ymd_hms(1998, 4, 3, 9, 0, 0),
            ymd_hms(1998, 5, 1, 9, 0, 0),
            ymd_hms(1998, 6, 5, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_first_friday_until_dec_24_1997() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970905T090000\n\
        RRULE:FREQ=MONTHLY;UNTIL=19971224T000000Z;BYDAY=1FR;COUNT=4", // TODO: remove `COUNT=4` when UNTIL is implemented
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 5, 9, 0, 0),
            ymd_hms(1997, 10, 3, 9, 0, 0),
            ymd_hms(1997, 11, 7, 9, 0, 0),
            ymd_hms(1997, 12, 5, 9, 0, 0),
        ]
    );
}

#[test]
fn every_other_month_on_the_first_and_last_sunday_of_the_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970907T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=2;COUNT=10;BYDAY=1SU,-1SU",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 7, 9, 0, 0),
            ymd_hms(1997, 9, 28, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
            ymd_hms(1997, 11, 30, 9, 0, 0),
            ymd_hms(1998, 1, 4, 9, 0, 0),
            ymd_hms(1998, 1, 25, 9, 0, 0),
            ymd_hms(1998, 3, 1, 9, 0, 0),
            ymd_hms(1998, 3, 29, 9, 0, 0),
            ymd_hms(1998, 5, 3, 9, 0, 0),
            ymd_hms(1998, 5, 31, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_second_to_last_monday_for_6_months() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970922T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=6;BYDAY=-2MO",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 22, 9, 0, 0),
            ymd_hms(1997, 10, 20, 9, 0, 0),
            ymd_hms(1997, 11, 17, 9, 0, 0),
            ymd_hms(1997, 12, 22, 9, 0, 0),
            ymd_hms(1998, 1, 19, 9, 0, 0),
            ymd_hms(1998, 2, 16, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_third_to_last_day_of_the_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970928T090000\n\
        RRULE:FREQ=MONTHLY;BYMONTHDAY=-3;COUNT=6",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 28, 9, 0, 0),
            ymd_hms(1997, 10, 29, 9, 0, 0),
            ymd_hms(1997, 11, 28, 9, 0, 0),
            ymd_hms(1997, 12, 29, 9, 0, 0),
            ymd_hms(1998, 1, 29, 9, 0, 0),
            ymd_hms(1998, 2, 26, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_2nd_and_15th_of_the_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=2,15",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 15, 9, 0, 0),
            ymd_hms(1997, 10, 2, 9, 0, 0),
            ymd_hms(1997, 10, 15, 9, 0, 0),
            ymd_hms(1997, 11, 2, 9, 0, 0),
            ymd_hms(1997, 11, 15, 9, 0, 0),
            ymd_hms(1997, 12, 2, 9, 0, 0),
            ymd_hms(1997, 12, 15, 9, 0, 0),
            ymd_hms(1998, 1, 2, 9, 0, 0),
            ymd_hms(1998, 1, 15, 9, 0, 0),
        ]
    );
}

#[test]
fn monthly_on_the_first_and_last_day_of_the_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970930T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=10;BYMONTHDAY=1,-1",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 10, 1, 9, 0, 0),
            ymd_hms(1997, 10, 31, 9, 0, 0),
            ymd_hms(1997, 11, 1, 9, 0, 0),
            ymd_hms(1997, 11, 30, 9, 0, 0),
            ymd_hms(1997, 12, 1, 9, 0, 0),
            ymd_hms(1997, 12, 31, 9, 0, 0),
            ymd_hms(1998, 1, 1, 9, 0, 0),
            ymd_hms(1998, 1, 31, 9, 0, 0),
            ymd_hms(1998, 2, 1, 9, 0, 0),
        ]
    );
}

#[test]
fn every_18_months_on_the_10th_through_15th_of_the_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970910T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=18;COUNT=10;BYMONTHDAY=10,11,12,13,14,15",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 10, 9, 0, 0),
            ymd_hms(1997, 9, 11, 9, 0, 0),
            ymd_hms(1997, 9, 12, 9, 0, 0),
            ymd_hms(1997, 9, 13, 9, 0, 0),
            ymd_hms(1997, 9, 14, 9, 0, 0),
            ymd_hms(1997, 9, 15, 9, 0, 0),
            ymd_hms(1999, 3, 10, 9, 0, 0),
            ymd_hms(1999, 3, 11, 9, 0, 0),
            ymd_hms(1999, 3, 12, 9, 0, 0),
            ymd_hms(1999, 3, 13, 9, 0, 0),
        ]
    );
}

#[test]
fn every_tuesday_ever_other_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;INTERVAL=2;BYDAY=TU;COUNT=18",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 9, 9, 0, 0),
            ymd_hms(1997, 9, 16, 9, 0, 0),
            ymd_hms(1997, 9, 23, 9, 0, 0),
            ymd_hms(1997, 9, 30, 9, 0, 0),
            ymd_hms(1997, 11, 4, 9, 0, 0),
            ymd_hms(1997, 11, 11, 9, 0, 0),
            ymd_hms(1997, 11, 18, 9, 0, 0),
            ymd_hms(1997, 11, 25, 9, 0, 0),
            ymd_hms(1998, 1, 6, 9, 0, 0),
            ymd_hms(1998, 1, 13, 9, 0, 0),
            ymd_hms(1998, 1, 20, 9, 0, 0),
            ymd_hms(1998, 1, 27, 9, 0, 0),
            ymd_hms(1998, 3, 3, 9, 0, 0),
            ymd_hms(1998, 3, 10, 9, 0, 0),
            ymd_hms(1998, 3, 17, 9, 0, 0),
            ymd_hms(1998, 3, 24, 9, 0, 0),
            ymd_hms(1998, 3, 31, 9, 0, 0),
        ]
    );
}

#[test]
fn yearly_in_june_and_july() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970610T090000\n\
        RRULE:FREQ=YEARLY;COUNT=10;BYMONTH=6,7",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 6, 10, 9, 0, 0),
            ymd_hms(1997, 7, 10, 9, 0, 0),
            ymd_hms(1998, 6, 10, 9, 0, 0),
            ymd_hms(1998, 7, 10, 9, 0, 0),
            ymd_hms(1999, 6, 10, 9, 0, 0),
            ymd_hms(1999, 7, 10, 9, 0, 0),
            ymd_hms(2000, 6, 10, 9, 0, 0),
            ymd_hms(2000, 7, 10, 9, 0, 0),
            ymd_hms(2001, 6, 10, 9, 0, 0),
            ymd_hms(2001, 7, 10, 9, 0, 0),
        ]
    );
}

#[test]
fn every_other_year_on_january_february_march() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970310T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=2;COUNT=10;BYMONTH=1,2,3",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 3, 10, 9, 0, 0),
            ymd_hms(1999, 1, 10, 9, 0, 0),
            ymd_hms(1999, 2, 10, 9, 0, 0),
            ymd_hms(1999, 3, 10, 9, 0, 0),
            ymd_hms(2001, 1, 10, 9, 0, 0),
            ymd_hms(2001, 2, 10, 9, 0, 0),
            ymd_hms(2001, 3, 10, 9, 0, 0),
            ymd_hms(2003, 1, 10, 9, 0, 0),
            ymd_hms(2003, 2, 10, 9, 0, 0),
            ymd_hms(2003, 3, 10, 9, 0, 0),
        ]
    );
}

#[test]
fn every_3rd_year_on_the_1st_100th_and_200th_day() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970101T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=3;COUNT=10;BYYEARDAY=1,100,200",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 1, 1, 9, 0, 0),
            ymd_hms(1997, 4, 10, 9, 0, 0),
            ymd_hms(1997, 7, 19, 9, 0, 0),
            ymd_hms(2000, 1, 1, 9, 0, 0),
            ymd_hms(2000, 4, 9, 9, 0, 0),
            ymd_hms(2000, 7, 18, 9, 0, 0),
            ymd_hms(2003, 1, 1, 9, 0, 0),
            ymd_hms(2003, 4, 10, 9, 0, 0),
            ymd_hms(2003, 7, 19, 9, 0, 0),
            ymd_hms(2006, 1, 1, 9, 0, 0),
        ]
    );
}

#[test]
fn every_20th_monday_of_the_year() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970519T090000\n\
        RRULE:FREQ=YEARLY;BYDAY=20MO;COUNT=3",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 5, 19, 9, 0, 0),
            ymd_hms(1998, 5, 18, 9, 0, 0),
            ymd_hms(1999, 5, 17, 9, 0, 0),
        ]
    );
}

#[test]
fn every_monday_in_the_20_week() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970512T090000\n\
        RRULE:FREQ=YEARLY;BYWEEKNO=20;BYDAY=MO;COUNT=3",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 5, 12, 9, 0, 0),
            ymd_hms(1998, 5, 11, 9, 0, 0),
            ymd_hms(1999, 5, 17, 9, 0, 0),
        ]
    );
}

#[test]
fn every_thursday_in_march() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970313T090000\n\
        RRULE:FREQ=YEARLY;BYMONTH=3;BYDAY=TH;COUNT=11",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 3, 13, 9, 0, 0),
            ymd_hms(1997, 3, 20, 9, 0, 0),
            ymd_hms(1997, 3, 27, 9, 0, 0),
            ymd_hms(1998, 3, 5, 9, 0, 0),
            ymd_hms(1998, 3, 12, 9, 0, 0),
            ymd_hms(1998, 3, 19, 9, 0, 0),
            ymd_hms(1998, 3, 26, 9, 0, 0),
            ymd_hms(1999, 3, 4, 9, 0, 0),
            ymd_hms(1999, 3, 11, 9, 0, 0),
            ymd_hms(1999, 3, 18, 9, 0, 0),
            ymd_hms(1999, 3, 25, 9, 0, 0),
        ]
    );
}

#[test]
fn every_thursday_during_june_july_august() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970605T090000\n\
        RRULE:FREQ=YEARLY;BYDAY=TH;BYMONTH=6,7,8;COUNT=39",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 6, 5, 9, 0, 0),
            ymd_hms(1997, 6, 12, 9, 0, 0),
            ymd_hms(1997, 6, 19, 9, 0, 0),
            ymd_hms(1997, 6, 26, 9, 0, 0),
            ymd_hms(1997, 7, 3, 9, 0, 0),
            ymd_hms(1997, 7, 10, 9, 0, 0),
            ymd_hms(1997, 7, 17, 9, 0, 0),
            ymd_hms(1997, 7, 24, 9, 0, 0),
            ymd_hms(1997, 7, 31, 9, 0, 0),
            ymd_hms(1997, 8, 7, 9, 0, 0),
            ymd_hms(1997, 8, 14, 9, 0, 0),
            ymd_hms(1997, 8, 21, 9, 0, 0),
            ymd_hms(1997, 8, 28, 9, 0, 0),
            ymd_hms(1998, 6, 4, 9, 0, 0),
            ymd_hms(1998, 6, 11, 9, 0, 0),
            ymd_hms(1998, 6, 18, 9, 0, 0),
            ymd_hms(1998, 6, 25, 9, 0, 0),
            ymd_hms(1998, 7, 2, 9, 0, 0),
            ymd_hms(1998, 7, 9, 9, 0, 0),
            ymd_hms(1998, 7, 16, 9, 0, 0),
            ymd_hms(1998, 7, 23, 9, 0, 0),
            ymd_hms(1998, 7, 30, 9, 0, 0),
            ymd_hms(1998, 8, 6, 9, 0, 0),
            ymd_hms(1998, 8, 13, 9, 0, 0),
            ymd_hms(1998, 8, 20, 9, 0, 0),
            ymd_hms(1998, 8, 27, 9, 0, 0),
            ymd_hms(1999, 6, 3, 9, 0, 0),
            ymd_hms(1999, 6, 10, 9, 0, 0),
            ymd_hms(1999, 6, 17, 9, 0, 0),
            ymd_hms(1999, 6, 24, 9, 0, 0),
            ymd_hms(1999, 7, 1, 9, 0, 0),
            ymd_hms(1999, 7, 8, 9, 0, 0),
            ymd_hms(1999, 7, 15, 9, 0, 0),
            ymd_hms(1999, 7, 22, 9, 0, 0),
            ymd_hms(1999, 7, 29, 9, 0, 0),
            ymd_hms(1999, 8, 5, 9, 0, 0),
            ymd_hms(1999, 8, 12, 9, 0, 0),
            ymd_hms(1999, 8, 19, 9, 0, 0),
            ymd_hms(1999, 8, 26, 9, 0, 0),
        ]
    );
}

#[test]
fn every_friday_the_13th() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MONTHLY;BYDAY=FR;BYMONTHDAY=13;COUNT=5",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1998, 2, 13, 9, 0, 0),
            ymd_hms(1998, 3, 13, 9, 0, 0),
            ymd_hms(1998, 11, 13, 9, 0, 0),
            ymd_hms(1999, 8, 13, 9, 0, 0),
            ymd_hms(2000, 10, 13, 9, 0, 0),
        ]
    );
}

#[test]
fn first_saturday_that_follows_the_first_sunday_of_month() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970913T090000\n\
        RRULE:FREQ=MONTHLY;BYDAY=SA;BYMONTHDAY=7,8,9,10,11,12,13;COUNT=10",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 13, 9, 0, 0),
            ymd_hms(1997, 10, 11, 9, 0, 0),
            ymd_hms(1997, 11, 8, 9, 0, 0),
            ymd_hms(1997, 12, 13, 9, 0, 0),
            ymd_hms(1998, 1, 10, 9, 0, 0),
            ymd_hms(1998, 2, 7, 9, 0, 0),
            ymd_hms(1998, 3, 7, 9, 0, 0),
            ymd_hms(1998, 4, 11, 9, 0, 0),
            ymd_hms(1998, 5, 9, 9, 0, 0),
            ymd_hms(1998, 6, 13, 9, 0, 0),
        ]
    );
}

#[test]
fn every_4_years_the_first_tuesday_after_a_monday_in_november() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19961105T090000\n\
        RRULE:FREQ=YEARLY;INTERVAL=4;BYMONTH=11;BYDAY=TU;BYMONTHDAY=2,3,4,5,6,7,8;COUNT=3",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1996, 11, 5, 9, 0, 0),
            ymd_hms(2000, 11, 7, 9, 0, 0),
            ymd_hms(2004, 11, 2, 9, 0, 0),
        ]
    );
}

#[ignore] // TODO: remove with BYSETPOS implemented
#[test]
fn third_instance_into_the_month_of_one_of_tuesday_wednesday_thursday() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970904T090000\n\
        RRULE:FREQ=MONTHLY;COUNT=3;BYDAY=TU,WE,TH;BYSETPOS=3",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 4, 9, 0, 0),
            ymd_hms(1997, 10, 7, 9, 0, 0),
            ymd_hms(1997, 11, 6, 9, 0, 0),
        ]
    );
}

#[test]
fn every_hour_and_half() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MINUTELY;INTERVAL=90;COUNT=4",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 10, 30, 0),
            ymd_hms(1997, 9, 2, 12, 0, 0),
            ymd_hms(1997, 9, 2, 13, 30, 0),
        ]
    );
}

#[test]
fn every_20_minutes_from_9_to_16_40_every_day_1() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=DAILY;BYHOUR=9,10,11,12,13,14,15,16;BYMINUTE=0,20,40;COUNT=25",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 20, 0),
            ymd_hms(1997, 9, 2, 9, 40, 0),
            ymd_hms(1997, 9, 2, 10, 0, 0),
            ymd_hms(1997, 9, 2, 10, 20, 0),
            ymd_hms(1997, 9, 2, 10, 40, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
            ymd_hms(1997, 9, 2, 11, 20, 0),
            ymd_hms(1997, 9, 2, 11, 40, 0),
            ymd_hms(1997, 9, 2, 12, 0, 0),
            ymd_hms(1997, 9, 2, 12, 20, 0),
            ymd_hms(1997, 9, 2, 12, 40, 0),
            ymd_hms(1997, 9, 2, 13, 0, 0),
            ymd_hms(1997, 9, 2, 13, 20, 0),
            ymd_hms(1997, 9, 2, 13, 40, 0),
            ymd_hms(1997, 9, 2, 14, 0, 0),
            ymd_hms(1997, 9, 2, 14, 20, 0),
            ymd_hms(1997, 9, 2, 14, 40, 0),
            ymd_hms(1997, 9, 2, 15, 0, 0),
            ymd_hms(1997, 9, 2, 15, 20, 0),
            ymd_hms(1997, 9, 2, 15, 40, 0),
            ymd_hms(1997, 9, 2, 16, 0, 0),
            ymd_hms(1997, 9, 2, 16, 20, 0),
            ymd_hms(1997, 9, 2, 16, 40, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
        ]
    );
}

#[test]
fn every_20_minutes_from_9_to_16_40_every_day_2() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970902T090000\n\
        RRULE:FREQ=MINUTELY;INTERVAL=20;BYHOUR=9,10,11,12,13,14,15,16;COUNT=25",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 9, 2, 9, 0, 0),
            ymd_hms(1997, 9, 2, 9, 20, 0),
            ymd_hms(1997, 9, 2, 9, 40, 0),
            ymd_hms(1997, 9, 2, 10, 0, 0),
            ymd_hms(1997, 9, 2, 10, 20, 0),
            ymd_hms(1997, 9, 2, 10, 40, 0),
            ymd_hms(1997, 9, 2, 11, 0, 0),
            ymd_hms(1997, 9, 2, 11, 20, 0),
            ymd_hms(1997, 9, 2, 11, 40, 0),
            ymd_hms(1997, 9, 2, 12, 0, 0),
            ymd_hms(1997, 9, 2, 12, 20, 0),
            ymd_hms(1997, 9, 2, 12, 40, 0),
            ymd_hms(1997, 9, 2, 13, 0, 0),
            ymd_hms(1997, 9, 2, 13, 20, 0),
            ymd_hms(1997, 9, 2, 13, 40, 0),
            ymd_hms(1997, 9, 2, 14, 0, 0),
            ymd_hms(1997, 9, 2, 14, 20, 0),
            ymd_hms(1997, 9, 2, 14, 40, 0),
            ymd_hms(1997, 9, 2, 15, 0, 0),
            ymd_hms(1997, 9, 2, 15, 20, 0),
            ymd_hms(1997, 9, 2, 15, 40, 0),
            ymd_hms(1997, 9, 2, 16, 0, 0),
            ymd_hms(1997, 9, 2, 16, 20, 0),
            ymd_hms(1997, 9, 2, 16, 40, 0),
            ymd_hms(1997, 9, 3, 9, 0, 0),
        ]
    );
}

#[test]
fn different_result_wkst_mo() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970805T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=4;BYDAY=TU,SU;WKST=MO",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 8, 5, 9, 0, 0),
            ymd_hms(1997, 8, 10, 9, 0, 0),
            ymd_hms(1997, 8, 19, 9, 0, 0),
            ymd_hms(1997, 8, 24, 9, 0, 0),
        ]
    );
}

// An example where the days generated makes a difference because of WKST: SU
#[test]
fn different_result_wkst_su() {
    let dts = test(
        "DTSTART;TZID=America/New_York:19970805T090000\n\
        RRULE:FREQ=WEEKLY;INTERVAL=2;COUNT=4;BYDAY=TU,SU;WKST=SU",
    );

    assert_eq!(
        dts,
        [
            ymd_hms(1997, 8, 5, 9, 0, 0),
            ymd_hms(1997, 8, 17, 9, 0, 0),
            ymd_hms(1997, 8, 19, 9, 0, 0),
            ymd_hms(1997, 8, 31, 9, 0, 0),
        ]
    );
}
