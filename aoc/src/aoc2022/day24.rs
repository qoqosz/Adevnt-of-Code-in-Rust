use aoc::{aoc, aoc_input};
use glam::IVec2;
use rustc_hash::FxHashSet;

type Array<T> = Vec<Vec<T>>;

trait Where<U> {
    type Item;

    fn r#where<F>(&self, r#where: F, x: U, y: U) -> Array<U>
    where
        F: Fn(&Self::Item) -> bool;
}

impl<T> Where<bool> for Array<T> {
    type Item = T;

    fn r#where<F>(&self, r#where: F, x: bool, y: bool) -> Array<bool>
    where
        F: Fn(&Self::Item) -> bool,
    {
        self.iter()
            .map(|row| {
                row.iter()
                    .map(|elem| match r#where(elem) {
                        true => x,
                        false => y,
                    })
                    .collect()
            })
            .collect()
    }
}

fn roll<T>(vec: &Vec<T>, shift: isize) -> Vec<T>
where
    T: Clone,
{
    match shift {
        1 => {
            let (last, elems) = vec.split_last().unwrap();
            let mut out = vec![last.clone()];
            out.extend_from_slice(elems);
            out
        }
        -1 => {
            let (first, elems) = vec.split_first().unwrap();
            let mut out = elems.to_vec();
            out.push(first.clone());
            out
        }
        _ => unreachable!(),
    }
}

trait Roll {
    fn roll(&self, shift: isize, axis: usize) -> Self;
}

impl<T> Roll for Array<T>
where
    T: Clone,
{
    fn roll(&self, shift: isize, axis: usize) -> Self {
        match axis {
            0 => roll(self, shift),
            1 => self.iter().map(|row| roll(row, shift)).collect(),
            _ => unreachable!(),
        }
    }
}

struct Valley {
    blizzards: [Array<bool>; 4],
    shape: [usize; 2],
    t: usize,
    start: IVec2,
    end: IVec2,
}

impl Valley {
    fn new(basin: &Array<char>) -> Self {
        let h = basin.len();
        let w: usize = basin.get(0).and_then(|x| Some(x.len())).unwrap_or_default();
        let blizzards = [
            basin.r#where(|ch| *ch == '>', true, false),
            basin.r#where(|ch| *ch == '<', true, false),
            basin.r#where(|ch| *ch == 'v', true, false),
            basin.r#where(|ch| *ch == '^', true, false),
        ];

        Self {
            blizzards,
            shape: [w, h],
            t: 0,
            start: IVec2::new(0, -1),
            end: IVec2::new(w as i32 - 1, h as i32),
        }
    }

    fn adj(&self, pos: IVec2) -> Vec<IVec2> {
        let poss = vec![
            pos,
            pos + IVec2::new(1, 0),
            pos + IVec2::new(-1, 0),
            pos + IVec2::new(0, 1),
            pos + IVec2::new(0, -1),
        ];

        poss.into_iter()
            .filter(|p| {
                *p == self.start
                    || *p == self.end
                    || ((0 <= p.x && p.x < self.shape[0] as i32)
                        && (0 <= p.y && p.y < self.shape[1] as i32))
            })
            .collect()
    }

    fn next(&self) -> [Array<bool>; 4] {
        [
            self.blizzards[0].roll(1, 1),
            self.blizzards[1].roll(-1, 1),
            self.blizzards[2].roll(1, 0),
            self.blizzards[3].roll(-1, 0),
        ]
    }

    fn process(&mut self) {
        self.t += 1;
        self.blizzards = self.next();
    }
}

fn bfs(valley: &mut Valley, start: IVec2, end: IVec2) -> Option<usize> {
    let mut queue = FxHashSet::from_iter([start]);

    loop {
        let next_blizz = valley.next();
        let mut _queue = FxHashSet::default();

        for pos in queue {
            for n in valley.adj(pos) {
                if n == end {
                    valley.process();
                    return Some(valley.t);
                }

                if n != start || n != end {
                    if next_blizz.iter().any(|arr| {
                        arr.get(n.y as usize)
                            .and_then(|row| Some(*row.get(n.x as usize).unwrap_or(&false)))
                            .unwrap_or(false)
                    }) {
                        continue;
                    }
                }

                _queue.insert(n);
            }
        }

        valley.process();
        queue = _queue;
    }
}

fn parse(data: &str) -> Array<char> {
    let basin = data
        .trim()
        .lines()
        .map(|line| line.chars().filter(|ch| *ch != '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let n = basin.len();

    basin[1..(n - 1)].to_vec()
}

#[aoc(2022, 24)]
pub fn main() {
    let data = aoc_input!(2022, 24).unwrap();
    let basin = parse(&data);
    let mut valley = Valley::new(&basin);
    let start = valley.start;
    let end = valley.end;

    // Part I
    let t1 = bfs(&mut valley, start, end).unwrap();
    println!("{t1}");

    // Part II
    let _ = bfs(&mut valley, end, start);
    let t2 = bfs(&mut valley, start, end).unwrap();
    println!("{t2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_where() {
        let x = vec![vec![0, 1, 0, 1], vec![1, 0, 0, 0]];
        let expected = vec![
            vec![false, true, false, true],
            vec![true, false, false, false],
        ];

        assert_eq!(x.r#where(|x| *x == 1, true, false), expected);
    }

    #[test]
    fn test_roll1() {
        let x = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected = vec![vec![5, 6], vec![1, 2], vec![3, 4]];

        assert_eq!(x.roll(1, 0), expected);
    }

    #[test]
    fn test_roll2() {
        let x = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected = vec![vec![3, 4], vec![5, 6], vec![1, 2]];

        assert_eq!(x.roll(-1, 0), expected);
    }

    #[test]
    fn test_roll3() {
        let x = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let expected = vec![vec![2, 1], vec![4, 3], vec![6, 5]];

        assert_eq!(x.roll(1, 1), expected);
    }

    #[test]
    fn test_part1() {
        let data = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let basin = parse(&data);
        let mut valley = Valley::new(&basin);
        let start = valley.start;
        let end = valley.end;

        // Part I
        let t1 = bfs(&mut valley, start, end).unwrap();
        assert_eq!(t1, 18);
    }
}
