#[macro_use]
extern crate lazy_static;

use aoc::aoc_input;
use aoc::point2d::Point;

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
    instructions
        .iter()
        .scan((Point::default(), 0), |(pos, phi), token| {
            let turn = token.chars().next().unwrap();
            let val = token[1..].parse::<i32>().unwrap();

            match turn {
                'R' => *phi += 1,
                _ => *phi += 3,
            };

            let ds = PHASES[*phi % 4];
            *pos = *pos + ds * val;
            Some(*pos)
        })
        .collect::<Vec<_>>()
}

fn main() {
    let data = aoc_input!(2016, 1).unwrap();
    let tokens = data.split(',').map(|x| x.trim()).collect::<Vec<_>>();

    // Part I
    let path = path(&tokens);
    let dest = path.last().unwrap();
    println!("{}", dest.dist());

    // Part II
    // let dup = first_dup(&path).unwrap();
    // println!("{}", dist(*dup));
}
