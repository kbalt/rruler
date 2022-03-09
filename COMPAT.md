# Known incompatibilities to other implementations

Comaprison against python's dateutil, rrule.js and rust-rrule

## Yearly

#### BYWEEKNO

Other implementations overflow the 53'th week as the first week of the next year if the current year only has 52 weeks.
Take the input:

```
DTSTART:19970902T090000Z
RRULE:FREQ=YEARLY;COUNT=10;INTERVAL=8;BYWEEKNO=53
```

This input requests all days in the 53'th week of every eighth week.

So the expected years are

- 1997
- 2005
- 2013
- 2021

But the output of other implementations is:

- 2005-01-01T09:00:00Z,
- 2005-01-02T09:00:00Z,
- 2021-01-01T09:00:00Z,
- 2021-01-02T09:00:00Z,
- 2021-01-03T09:00:00Z,
- 2037-12-28T09:00:00Z,
- 2037-12-29T09:00:00Z,
- 2037-12-30T09:00:00Z,
- 2037-12-31T09:00:00Z,

The years 2005 and 2021 yield dates from the last week of the previous year,
which arguably are NOT part the 53th week of 2005/2013/2021. One could argue that the 53'th week of 
2005 would overflow to the 1st week of 2006 but `rruler` currently checks that the BYWEEKNO parameter
is less or equal to the weeks in a year.
Otherwise evaluating the negative BYWEEKNO would lead to wrong results.

`rruler` output:

- 2037-12-28T09:00:00Z,
- 2037-12-29T09:00:00Z,
- 2037-12-30T09:00:00Z,
- 2037-12-31T09:00:00Z,
- 2038-01-01T09:00:00Z,
- 2038-01-02T09:00:00Z,
- 2038-01-03T09:00:00Z,
- 2093-12-28T09:00:00Z,
- 2093-12-29T09:00:00Z,
- 2093-12-30T09:00:00Z,
- 2061-01-01T09:00:00Z,
