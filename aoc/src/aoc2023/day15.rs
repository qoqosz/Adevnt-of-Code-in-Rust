use aoc::{aoc, aoc_input};
use std::hash::{Hash, Hasher};

#[derive(Default)]
struct LensHasher(u8);

impl Hasher for LensHasher {
    fn write(&mut self, bytes: &[u8]) {
        let val = bytes.iter().fold(self.0, |acc, x| {
            ((acc as usize + *x as usize) * 17 % 256) as u8
        });
        *self = LensHasher(val);
    }

    fn finish(&self) -> u64 {
        self.0 as u64
    }
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal: Option<u8>,
}

impl<'a> Lens<'a> {
    fn index(&self) -> usize {
        let mut hasher = LensHasher::default();
        self.hash(&mut hasher);
        hasher.finish() as usize
    }
}

impl<'a> Hash for Lens<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write(self.label.as_bytes());
    }
}

impl<'a> From<&'a str> for Lens<'a> {
    fn from(s: &'a str) -> Self {
        match s.split_once('=') {
            Some((label, f)) => Self {
                label,
                focal: f.parse::<u8>().ok(),
            },
            _ => Self {
                label: s.trim_end_matches('-'),
                focal: None,
            },
        }
    }
}

fn parse(data: &str) -> Vec<&str> {
    data.trim().split(',').filter(|x| !x.is_empty()).collect()
}

fn box_fill<'a>(data: &[&'a str], boxes: &'a mut Vec<Vec<Lens<'a>>>) -> &'a Vec<Vec<Lens<'a>>> {
    for lens in data.iter().map(|&x| Lens::from(x)) {
        let idx = lens.index();

        match lens.focal {
            Some(_) => {
                // check if there is already a lens with this label
                match boxes[idx].iter_mut().find(|l| l.label == lens.label) {
                    Some(l) => l.focal = lens.focal,
                    _ => boxes[idx].push(lens),
                }
            }
            _ => {
                // remove the label from a box
                boxes[idx].retain(|l| l.label != lens.label);
            }
        }
    }

    boxes
}

fn power(boxes: &[Vec<Lens>]) -> usize {
    boxes
        .iter()
        .enumerate()
        .flat_map(|(i, b)| {
            b.iter()
                .flat_map(|l| l.focal)
                .enumerate()
                .map(move |(j, f)| (i + 1) * (j + 1) * f as usize)
        })
        .sum()
}

#[aoc(2023, 15)]
pub fn main() {
    let data = aoc_input!(2023, 15).unwrap();
    let input = parse(&data);

    // Part I
    let res: u64 = input
        .iter()
        .map(|x| {
            let mut hasher = LensHasher::default();
            hasher.write(x.as_bytes());
            hasher.finish()
        })
        .sum();
    println!("{res}");

    // Part II
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    let boxes = box_fill(&input, &mut boxes);
    println!("{}", power(boxes));
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash1() {
        let mut hasher = LensHasher::default();
        hasher.write("HASH".as_bytes());
        assert_eq!(hasher.finish(), 52);
    }

    #[test]
    fn test_hash2() {
        let mut hasher = LensHasher::default();
        hasher.write("rn=1".as_bytes());
        assert_eq!(hasher.finish(), 30);
    }

    #[test]
    fn test_part1() {
        let input = parse(EXAMPLE);
        let res: u64 = input
            .iter()
            .map(|x| {
                let mut hasher = LensHasher::default();
                hasher.write(x.as_bytes());
                hasher.finish()
            })
            .sum();
        assert_eq!(res, 1320);
    }

    #[test]
    fn test_p2() {
        let input = parse(EXAMPLE);
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 4];
        let boxes = box_fill(&input, &mut boxes);
        assert_eq!(power(boxes), 145);
    }
}
