use aoc::{aoc, aoc_input};
use atoi::atoi;
use glam::IVec2;
use rustc_hash::FxHashSet;

#[inline]
fn sign(x: i32) -> i32 {
    match x.cmp(&0) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}

trait Sign {
    fn sign(&self) -> Self;
}

impl Sign for IVec2 {
    fn sign(&self) -> Self {
        IVec2::new(sign(self.x), sign(self.y))
    }
}

enum Direction {
    Down,
    Up,
    Left,
    Right,
}

impl Direction {
    fn to_ivec2(&self) -> IVec2 {
        match self {
            Self::Down => IVec2::new(0, -1),
            Self::Up => IVec2::new(0, 1),
            Self::Left => IVec2::new(-1, 0),
            Self::Right => IVec2::new(1, 0),
        }
    }
}

struct Motion(Direction, i32);

impl From<&str> for Motion {
    fn from(value: &str) -> Self {
        let (dir, val) = value.split_once(' ').unwrap();
        let val = atoi::<i32>(val.as_bytes()).unwrap();
        let dir = match dir {
            "D" => Direction::Down,
            "U" => Direction::Up,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => unimplemented!(),
        };
        Self(dir, val)
    }
}

#[allow(non_snake_case)]
struct Rope {
    t_visited: FxHashSet<IVec2>,
    H: IVec2,
    T: IVec2,
}

impl Rope {
    fn new() -> Self {
        let mut t_visited = FxHashSet::default();
        t_visited.reserve(10_000);
        t_visited.insert(IVec2::default());

        Self {
            t_visited,
            H: IVec2::default(),
            T: IVec2::default(),
        }
    }

    #[inline]
    fn step(&mut self, direction: &Direction) {
        self.move_head(direction);
        self.move_tail();
    }

    #[inline]
    fn move_head(&mut self, direction: &Direction) {
        self.H += direction.to_ivec2();
    }

    #[inline]
    fn move_tail(&mut self) {
        let delta = self.H - self.T;

        if delta.as_vec2().length() > 1.5 {
            self.T += delta.sign();
            self.t_visited.insert(self.T);
        }
    }
}

fn sim_rope(motions: &[Motion], n_knots: usize) -> usize {
    let mut rope = (0..(n_knots - 1)).map(|_| Rope::new()).collect::<Vec<_>>();

    for step in motions {
        for _ in 0..step.1 {
            rope[0].step(&step.0);

            for k in 1..(n_knots - 1) {
                rope[k].H = rope[k - 1].T;
                rope[k].move_tail();
            }
        }
    }

    rope.last().unwrap().t_visited.len()
}

#[aoc(2022, 9)]
pub fn main() {
    let data = aoc_input!(2022, 9).unwrap();
    let motions = data
        .trim_end()
        .lines()
        .map(Motion::from)
        .collect::<Vec<_>>();

    // Part I
    println!("{}", sim_rope(&motions, 2));

    // Part II
    println!("{}", sim_rope(&motions, 10));
}
