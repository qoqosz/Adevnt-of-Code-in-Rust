use aoc::{aoc, aoc_input, counter::Counter};
use itertools::Itertools;
use std::{fmt::Debug, num::ParseIntError};

#[derive(Debug)]
enum Event {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

impl TryFrom<&str> for Event {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.contains("wakes") {
            return Ok(Self::WakeUp);
        } else if value.contains("falls") {
            return Ok(Self::FallAsleep);
        } else {
            let (_, line) = value.split_once('#').ok_or(())?;
            let (num, _) = line.split_once(' ').ok_or(())?;
            let id = num.parse().map_err(|_| ())?;
            Ok(Self::BeginShift(id))
        }
    }
}

#[derive(Clone)]
struct DateTime {
    date: (usize, usize, usize),
    time: (usize, usize),
}

impl DateTime {
    fn tuple(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.date.0,
            self.date.1,
            self.date.2,
            self.time.0,
            self.time.1,
        )
    }

    fn minute(&self) -> usize {
        self.time.1
    }
}

impl From<(usize, usize, usize, usize, usize)> for DateTime {
    fn from(value: (usize, usize, usize, usize, usize)) -> Self {
        Self {
            date: (value.0, value.1, value.2),
            time: (value.3, value.4),
        }
    }
}

impl TryFrom<&str> for DateTime {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok((
            value[0..4].parse()?,
            value[5..7].parse()?,
            value[8..10].parse()?,
            value[11..13].parse()?,
            value[14..16].parse()?,
        )
            .into())
    }
}

impl Debug for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:04}-{:02}-{:02} {:02}:{:02}",
            self.date.0, self.date.1, self.date.2, self.time.0, self.time.1
        )
    }
}

impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.tuple().eq(&other.tuple())
    }
}

impl Eq for DateTime {}

impl Ord for DateTime {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tuple().cmp(&other.tuple())
    }
}

impl PartialOrd for DateTime {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tuple().partial_cmp(&other.tuple())
    }
}

#[derive(Debug)]
struct Record {
    datetime: DateTime,
    event: Event,
}

impl TryFrom<&str> for Record {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (dt, event) = value.split_once("] ").ok_or(())?;
        Ok(Self {
            datetime: DateTime::try_from(&dt[1..]).map_err(|_| ())?,
            event: Event::try_from(event)?,
        })
    }
}

enum Strategy {
    Strategy1,
    Strategy2,
}

impl Strategy {
    fn solve(&self, freqs: &Counter<(usize, usize)>) -> Option<usize> {
        match self {
            Strategy::Strategy1 => {
                // key: guard's id, val: total time asleep
                let guards = freqs
                    .into_iter()
                    .map(|((id, _), cnt)| (*id, *cnt))
                    .collect::<Counter<usize>>();

                if let Some((guard_id, _)) = guards.into_iter().max_by_key(|(_, cnt)| *cnt) {
                    return freqs
                        .into_iter()
                        .filter(|((id, _), _)| *id == guard_id)
                        .max_by_key(|((_, _), cnt)| **cnt)
                        .map(|((_, minute), _)| guard_id * minute);
                }
            }
            Strategy::Strategy2 => {
                return freqs
                    .into_iter()
                    .max_by_key(|(_, v)| **v)
                    .map(|((id, minute), _)| id * minute);
            }
        }
        None
    }
}

fn solve(records: &[Record], strategy: Strategy) -> Option<usize> {
    // freqs: (id, minute) -> count asleep
    let mut freqs = Counter::<(usize, usize)>::default();
    let mut current_id = 0;
    let mut t0 = 0;

    for record in records {
        match record.event {
            Event::BeginShift(id) => current_id = id,
            Event::FallAsleep => t0 = record.datetime.minute(),
            Event::WakeUp => {
                for t in t0..(record.datetime.minute()) {
                    freqs.increment((current_id, t));
                }
            }
        }
    }

    strategy.solve(&freqs)
}

#[aoc(2018, 4)]
pub fn main() {
    let data = aoc_input!(2018, 4).unwrap();
    let records = data
        .lines()
        .flat_map(|line| Record::try_from(line))
        .sorted_unstable_by_key(|r| r.datetime.clone())
        .collect::<Vec<_>>();

    // Part I
    println!("{}", solve(&records, Strategy::Strategy1).unwrap());

    // Part II
    println!("{}", solve(&records, Strategy::Strategy2).unwrap());
}
