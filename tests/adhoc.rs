use rruler::iter::RRuleIter;
use rruler::rrule::RRule;
use std::io::Write;

#[test]
fn adhoc() {
    let (rem, rrule) = RRule::parse(
        "DTSTART;TZID=America/New_York:20200101T000000\nRRULE:FREQ=YEARLY;BYMONTH=1,2,3,4,5,6;BYDAY=-2MO",
    )
    .unwrap();
    assert_eq!(rem, "");

    rrule.verify(true).unwrap();

    let iter = RRuleIter::new(&rrule);

    dbg!(&iter);

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();

    for (i, item) in iter.enumerate() {
        // writeln!(stdout, "{:?}", item).unwrap();
        if i % 1000 == 0 {
            writeln!(stdout, "{:?}", item).unwrap();
        }
    }
}
