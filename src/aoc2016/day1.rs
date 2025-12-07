use aoc::aoc_input;
use aoc::point2d::Point;
use lazy_static::lazy_static;

lazy_static! {
    static ref PHASES: [Point; 4] = [
        Point::new(0, 1),
        Point::new(1, 0),
        Point::new(0, -1),
        Point::new(-1, 0),
    ];
}

trait Dist {
    fn dist(&self) -> i32;
}

impl Dist for Point {
    fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn path(instructions: &[&str]) -> Vec<Point> {
    let mut pos = Point::default();
    let mut out = vec![pos];
    let mut phi = 0;

    for token in instructions {
        let turn = token.chars().next().unwrap();
        let val = token[1..].parse::<usize>().unwrap();

        match turn {
            'R' => phi += 1,
            _ => phi += 3,
        };

        let ds = PHASES[phi % 4];

        for _ in 1..=val {
            pos = pos + ds;
            out.push(pos);
        }
    }

    out
}

fn find_duplicate(path: &[Point]) -> Option<Point> {
    let n = path.len();

    for i in 0..n {
        for j in (i + 1)..n {
            if path[i] == path[j] {
                return Some(path[i]);
            }
        }
    }
    None
}

fn main() {
    let data = aoc_input!(2016, 1).unwrap();
    let tokens = data.split(',').map(|x| x.trim()).collect::<Vec<_>>();

    // Part I
    let path = path(&tokens);
    let dest = path.last().unwrap();
    println!("{}", dest.dist());

    // Part II
    let dup = find_duplicate(&path).unwrap();
    println!("{}", dup.dist());
}
