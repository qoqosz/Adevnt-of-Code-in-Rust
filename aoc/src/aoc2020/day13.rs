use aoc::{aoc, aoc_input};

fn parse(data: &str) -> (u64, Vec<Option<u64>>) {
    let mut lines = data.lines();
    let timestamp = lines.next().unwrap().parse::<u64>().unwrap();
    let ids = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().ok())
        .collect();

    (timestamp, ids)
}

// Part I
fn find_bus(timestamp: u64, ids: &[Option<u64>]) -> Option<u64> {
    ids.iter()
        .flatten()
        .map(|&id| {
            let t = (timestamp / id + 1) * id;
            (id, t)
        })
        .min_by_key(|(_, t)| *t)
        .map(|(id, t)| id * (t - timestamp))
}

// Part II - Chinese remainder theorem
fn crt_find(eqs: &[(u64, u64)]) -> Option<u64> {
    let mut iter = eqs.iter();
    let (mut a, mut b) = *iter.next()?;

    for &(c, d) in iter {
        let t = (0..).find(|&i| (a + b * i) % d == c)?;
        (a, b) = (a + b * t, b * d);
    }

    Some(a)
}

#[aoc(2020, 13)]
pub fn main() {
    let data = aoc_input!(2020, 13).unwrap();
    let (timestamp, ids) = parse(&data);

    // Part I
    println!("{}", find_bus(timestamp, &ids).unwrap());

    // Part II
    let eqs = ids
        .iter()
        .enumerate()
        .filter(|x| x.1.is_some())
        .map(|(i, x)| (i as u64, x.unwrap()))
        .map(|(i, x)| ((x - (i % x)) % x, x))
        .collect::<Vec<_>>();
    println!("{}", crt_find(&eqs).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() {
        let (timestamp, ids) = parse(EXAMPLE);
        assert_eq!(find_bus(timestamp, &ids), Some(295));
    }

    #[test]
    fn test_part2() {
        let (_, ids) = parse(EXAMPLE);
        let data = ids
            .iter()
            .enumerate()
            .filter(|x| x.1.is_some())
            .map(|(i, x)| {
                let x = x.unwrap();
                ((x - i as u64) % x, x)
            })
            .collect::<Vec<_>>();

        assert_eq!(1068781, crt_find(&data).unwrap());
    }
}
