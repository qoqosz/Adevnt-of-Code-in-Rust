use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Entry<'a> {
    name: Rc<str>,
    sector: usize,
    checksum: &'a str,
}

impl<'a> TryFrom<&'a str> for Entry<'a> {
    type Error = ();

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if let Some((name_sector, checksum)) = value.trim_end_matches(']').split_once('[') {
            if let Some((name, sector)) = name_sector.rsplit_once('-') {
                return Ok(Entry {
                    name: Rc::from(name),
                    sector: sector.parse::<usize>().or_else(|_| Err(()))?,
                    checksum,
                });
            }
        }
        Err(())
    }
}

impl<'a> Entry<'a> {
    fn get_counts(&self) -> Vec<(usize, char)> {
        let mut counts = FxHashMap::default();
        for ch in self.name.chars().filter(|ch| *ch != '-') {
            counts.entry(ch).and_modify(|cnt| *cnt += 1).or_insert(1);
        }
        counts
            .iter()
            .map(|(k, v)| (*v, *k))
            .sorted_by_key(|x| (-(x.0 as i32), x.1 as i32))
            .collect::<Vec<_>>()
    }

    fn is_room(&self) -> bool {
        let hash = self
            .get_counts()
            .iter()
            .take(5)
            .map(|x| x.1)
            .collect::<String>();
        &hash == self.checksum
    }

    fn decrypt(&self) -> Entry<'a> {
        let shift = (self.sector % 26) as u8;
        let name = self
            .name
            .chars()
            .filter(|ch| *ch != '-')
            .map(|ch| {
                let ord = ch as u8 - b'a';
                (((ord + shift) % 26) + b'a') as char
            })
            .collect::<String>();

        Entry {
            name: Rc::from(name),
            ..*self
        }
    }
}

#[aoc(2016, 4)]
pub fn main() {
    let data = aoc_input!(2016, 4).unwrap();
    let entries = data
        .lines()
        .flat_map(|l| Entry::try_from(l))
        .collect::<Vec<_>>();
    let mut rooms = entries.iter().filter(|e| e.is_room());

    // Part I
    let sector_ids = rooms.clone().map(|e| e.sector).sum::<usize>();
    println!("{sector_ids}");

    // Part II
    let sector_id = rooms
        .find(|e| e.decrypt().name.starts_with("northpole"))
        .map(|e| e.sector)
        .unwrap();
    println!("{sector_id}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counts() {
        let entry = Entry::try_from("aaaaa-bbb-z-y-x-123[abxyz]").unwrap();
        // println!("{:?}", entry.get_counts());
    }
}
