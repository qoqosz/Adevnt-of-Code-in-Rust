use aoc::{aoc, aoc_input};
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;
use std::fmt;
use std::hash::Hash;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct Point3 {
    x: u16,
    y: u16,
    z: u16,
}

impl fmt::Debug for Point3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl From<(u16, u16, u16)> for Point3 {
    fn from(value: (u16, u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
            z: value.2,
        }
    }
}

impl Ord for Point3 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = (self.z, self.x, self.y);
        let rhs = (other.z, other.x, other.y);
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Point3 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point2 {
    x: u16,
    y: u16,
}

impl fmt::Debug for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2({}, {})", self.x, self.y)
    }
}

impl From<(u16, u16)> for Point2 {
    fn from(value: (u16, u16)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl From<Point3> for Point2 {
    fn from(value: Point3) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Brick {
    ends: [Point3; 2],
}

impl fmt::Debug for Brick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Brick[{:?} - {:?}]", self.ends[0], self.ends[1])
    }
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once('~').unwrap();
        let to_point = |x: &str| -> Point3 {
            x.split(',')
                .flat_map(|x| x.parse::<u16>())
                .collect_tuple::<(_, _, _)>()
                .unwrap()
                .into()
        };
        let mut left = to_point(left);
        let mut right = to_point(right);

        if right < left {
            std::mem::swap(&mut left, &mut right);
        }

        Self {
            ends: [left, right],
        }
    }
}

impl Brick {
    fn iter_xy(&self) -> impl Iterator<Item = Point2> + '_ {
        self.into_iter()
            .map(std::convert::Into::<Point2>::into)
            .unique()
    }

    fn settle(&mut self, z: u16) {
        let dz = self.ends[0].z - z;
        self.ends[0].z = z;
        self.ends[1].z -= dz;
    }
}

impl<'a> IntoIterator for &'a Brick {
    type Item = Point3;
    type IntoIter = BrickIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        BrickIterator::new(self)
    }
}

struct BrickIterator<'a> {
    brick: &'a Brick,
    current: Option<Point3>,
}

impl<'a> BrickIterator<'a> {
    fn new(brick: &'a Brick) -> Self {
        Self {
            brick,
            current: Some(brick.ends[0]),
        }
    }
}

impl<'a> Iterator for BrickIterator<'a> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        let end = self.brick.ends[1];

        let Some(ret) = self.current else {
            return None;
        };
        if ret.x < end.x {
            self.current = Some(Point3 {
                x: ret.x + 1,
                ..ret
            });
        } else if ret.y < end.y {
            self.current = Some(Point3 {
                y: ret.y + 1,
                ..ret
            });
        } else if ret.z < end.z {
            self.current = Some(Point3 {
                z: ret.z + 1,
                ..ret
            });
        } else {
            self.current = None;
        }

        Some(ret)
    }
}

#[derive(Debug, Default)]
struct TopView {
    grid: FxHashMap<Point2, (usize, u16)>,
}

impl TopView {
    fn get(&self, key: &Point2) -> Option<&(usize, u16)> {
        self.grid.get(key)
    }

    fn place(&mut self, idx: usize, brick: &Brick) {
        let z = brick.ends[1].z;

        for p in brick.iter_xy() {
            self.grid.insert(p, (idx, z));
        }
    }

    fn max_height(&self, brick: &Brick) -> u16 {
        brick
            .iter_xy()
            .flat_map(|p| self.get(&p))
            .map(|(_, h)| *h)
            .max()
            .unwrap_or(0)
    }
}

#[derive(Default)]
struct Tower {
    bricks: Vec<Brick>,
    supported: Vec<Vec<usize>>,
    supporting: Vec<Vec<usize>>,
    top_view: TopView,
}

impl Tower {
    fn from_bricks(bricks: &[Brick]) -> Self {
        let mut tower = Tower::default();
        bricks.iter().for_each(|b| tower.fall_brick(b));
        tower
    }

    fn fall_brick(&mut self, brick: &Brick) {
        let mut brick = *brick;
        let idx = self.bricks.len();

        // settle brick
        let height = self.top_view.max_height(&brick);
        brick.settle(height + 1);

        // get supported
        let support = brick
            .iter_xy()
            .flat_map(|p| self.top_view.get(&p))
            .filter(|(_, h)| *h == height)
            .map(|(idx, _)| *idx)
            .unique()
            .collect::<Vec<_>>();

        // fill supporting
        for i in &support {
            self.supporting[*i].push(idx);
        }

        // update self
        self.top_view.place(idx, &brick);
        self.bricks.push(brick);
        self.supported.push(support);
        self.supporting.push(vec![]);
    }

    // block can be disintegrated if all supporting blocks
    // are supported by more than 1 block
    fn can_disintegrate(&self, i: usize) -> bool {
        self.supporting[i]
            .iter()
            .all(|&j| self.supported[j].len() > 1)
    }

    // part 2
    fn chain_reaction(&self, i: usize) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = FxHashSet::default();
        queue.push_back(i);

        while let Some(node) = queue.pop_front() {
            if !visited.insert(node) {
                continue;
            }

            for &next in &self.supporting[node] {
                if self.supported[next]
                    .iter()
                    .filter(|&n| !visited.contains(n))
                    .count()
                    == 0
                {
                    queue.push_back(next);
                }
            }
        }
        visited.len().saturating_sub(1)
    }
}

impl fmt::Debug for Tower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tower {{")?;
        writeln!(f, "  bricks: [")?;

        for brick in &self.bricks {
            writeln!(f, "    {:?},", brick)?;
        }

        writeln!(f, "  ],")?;
        writeln!(f, "  supported: [")?;

        for sup in &self.supported {
            writeln!(f, "    {:?},", sup)?;
        }

        writeln!(f, "  ],")?;
        writeln!(f, "  supporting: [")?;

        for sup in &self.supporting {
            writeln!(f, "    {:?},", sup)?;
        }

        writeln!(f, "  ],")?;
        writeln!(f, "  top_view: [")?;

        for (k, v) in self.top_view.grid.iter().sorted_unstable_by_key(|x| *x.0) {
            writeln!(f, "    {:?}: {:?},", k, v)?;
        }

        writeln!(f, "  ]\n}}")?;

        Ok(())
    }
}

fn parse(data: &str) -> Tower {
    let mut bricks = data.trim().lines().map(Brick::from).collect::<Vec<_>>();
    bricks.sort_unstable();
    Tower::from_bricks(&bricks)
}

#[aoc(2023, 22)]
pub fn main() {
    let data = aoc_input!(2023, 22).unwrap();
    let tower = parse(&data);
    let n = tower.bricks.len();

    // Part I
    let count = (0..n).filter(|&i| tower.can_disintegrate(i)).count();
    println!("{count}");

    // Part II
    let sum: usize = (0..n).map(|i| tower.chain_reaction(i)).sum();
    println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

    #[test]
    fn test_into_point3() {
        let x: Point3 = (2, 8, 9).into();
        let y = Point3 { x: 2, y: 8, z: 9 };
        assert_eq!(x, y);
    }

    #[test]
    fn test_parse_brick() {
        let line = "2,2,5~2,0,5";
        let brick = Brick::from(line);
        let ends = [(2, 0, 5).into(), (2, 2, 5).into()];
        assert_eq!(brick.ends, ends);
    }

    #[test]
    fn test_brick_iter() {
        let brick = Brick::from("2,2,5~2,0,5");
        let points = brick.into_iter().collect::<Vec<_>>();
        let expected = vec![(2, 0, 5).into(), (2, 1, 5).into(), (2, 2, 5).into()];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_brick_iter_z() {
        let brick = Brick::from("1,1,8~1,1,9");
        let points = brick.into_iter().collect::<Vec<_>>();
        let expected = vec![(1, 1, 8).into(), (1, 1, 9).into()];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_brick_iter_xy() {
        let brick = Brick::from("1,1,8~1,1,9");
        let points = brick.iter_xy().collect::<Vec<_>>();
        let expected = vec![(1, 1).into()];
        assert_eq!(points, expected);
    }

    #[test]
    fn test_brick_settle() {
        let mut brick = Brick::from("1,1,8~1,1,9");
        brick.settle(2);
        assert_eq!(brick.ends[0].z, 2);
        assert_eq!(brick.ends[1].z, 3);
    }

    #[test]
    fn test_top_view_place() {
        let mut top_view = TopView::default();
        let brick = Brick::from("1,2,8~1,4,8");
        top_view.place(0, &brick);
        let expected = FxHashMap::from_iter([
            ((1, 2).into(), (0, 8)),
            ((1, 3).into(), (0, 8)),
            ((1, 4).into(), (0, 8)),
        ]);
        assert_eq!(top_view.grid, expected);
    }

    #[test]
    fn test_simple_tower() {
        let brick1 = Brick::from("1,0,2~1,2,2");
        let brick2 = Brick::from("1,2,3~3,2,3");
        let tower = Tower::from_bricks(&[brick1, brick2]);
        let expected = FxHashMap::from_iter([
            ((1, 0).into(), (0, 1)),
            ((1, 1).into(), (0, 1)),
            ((1, 2).into(), (1, 2)),
            ((2, 2).into(), (1, 2)),
            ((3, 2).into(), (1, 2)),
        ]);
        assert_eq!(tower.top_view.grid, expected);
        assert_eq!(tower.supported, vec![vec![], vec![0]]);
    }

    #[test]
    fn test_part1() {
        let tower = parse(EXAMPLE);
        let n = tower.bricks.len();
        let count = (0..n).filter(|&i| tower.can_disintegrate(i)).count();
        assert_eq!(count, 5);
    }

    #[test]
    fn test_part2() {
        let tower = parse(EXAMPLE);
        let n = tower.bricks.len();
        let sum: usize = (0..n).map(|i| tower.chain_reaction(i)).sum();
        assert_eq!(sum, 7);
    }
}
