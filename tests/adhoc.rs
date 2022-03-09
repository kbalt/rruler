use chrono::TimeZone;
use chrono_tz::Tz;
use rruler::iter::{RRuleIter, RRuleIterYield};
use rruler::rrule::RRule;
use std::io::Write;
use std::str::FromStr;

#[test]
fn adhoc() {
    let input = "DTSTART:20200101T000000Z\n\
    RRULE:FREQ=YEARLY;UNTIL=20830311T000000Z;COUNT=30;INTERVAL=1;WKST=MO;BYMONTH=1;BYYEARDAY=2;BYWEEKNO=1";

    let rrule = RRule::from_str(input).unwrap();
    rrule.verify(true).unwrap();
    let iter = RRuleIter::new(&rrule);

    let rrule2 = rrule::RRule::from_str(input).unwrap();
    let iter2 = rrule2.into_iter();

    // #######################################################
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    let mut mismatch = vec![];

    for (i, (mine, theirs)) in iter.zip(iter2).take(99999999999).enumerate() {
        let mine = match mine {
            RRuleIterYield::DateTimeLocal(x) => Tz::UTC.from_utc_datetime(&x),
            RRuleIterYield::DateTimeTz(x) => x,
        };

        let mine = mine;
        let theirs = theirs;

        writeln!(stdout, "{}\t{mine} == {theirs}", i + 1).unwrap();

        if mine != theirs {
            mismatch.push((mine, theirs));
            if mismatch.len() > 10 {
                break;
            }
        }
    }

    println!("{:#?}", mismatch);
}
