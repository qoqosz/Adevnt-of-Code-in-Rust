use aoc::{aoc, aoc_input};

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focal: Option<u8>,
}

impl<'a> Lens<'a> {
    fn index(&self) -> usize {
        hash(self.label)
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

fn hash(text: &str) -> usize {
    text.as_bytes()
        .iter()
        .fold(0, |acc, x| (acc + *x as usize) * 17 % 256)
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
    let res: usize = input.iter().map(|x| hash(x)).sum();
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
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
    }

    #[test]
    fn test_part1() {
        let input = parse(EXAMPLE);
        let res: usize = input.iter().map(|x| hash(x)).sum();
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
