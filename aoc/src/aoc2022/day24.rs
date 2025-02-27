use aoc::{aoc, aoc_input};
use glam::IVec2;

type Array<T> = Vec<Vec<T>>;
static MOTIONS: [char; 4] = ['>', '<', 'v', '^'];

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
            start: IVec2::new(-1, 0),
            end: IVec2::new(w as i32, h as i32 - 1),
        }
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

    println!("{basin:?}");
}
