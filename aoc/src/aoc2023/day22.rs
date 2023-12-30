use aoc::{aoc, aoc_input};
use itertools::Itertools;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point3 {
    x: u16,
    y: u16,
    z: u16,
}

impl Point3 {
    fn new(x: u16, y: u16, z: u16) -> Self {
        Self { x, y, z }
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

#[derive(Debug)]
struct Brick {
    ends: [Point3; 2],
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
        } else {
            self.current = None;
        }

        Some(ret)
    }
}

fn parse(data: &str) {
    _ = data.trim().lines();
}

#[aoc(2023, 22)]
pub fn main() {
    let data = aoc_input!(2023, 22).unwrap();
    parse(&data);

    // Part I

    // Part II
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
        let y = Point3::new(2, 8, 9);
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
    fn test_part1() {
        parse(EXAMPLE);
    }

    #[test]
    fn test_part2() {
        //        assert_eq!(part2(&lines), 1);
    }
}
