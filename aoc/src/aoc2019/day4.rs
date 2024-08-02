use aoc::aoc;
use itertools::Itertools;

trait Password {
    fn is_valid(&self) -> bool;
    fn is_valid2(&self) -> bool;
}

impl Password for u32 {
    fn is_valid(&self) -> bool {
        let mut is_adj_same = false;

        for (x, y) in self.to_string().chars().tuple_windows() {
            let (elem, next) = (x as u32, y as u32);

            if elem == next {
                is_adj_same = true;
            } else if elem > next {
                return false;
            }
        }

        is_adj_same
    }

    fn is_valid2(&self) -> bool {
        let mut is_adj_same = false;
        let mut adj_num = 0;

        for (x, y) in self.to_string().chars().tuple_windows() {
            let (elem, next) = (x as u32, y as u32);

            if elem == next {
                adj_num += 1;
            } else if elem > next {
                return false;
            } else {
                if adj_num == 1 {
                    is_adj_same = true;
                }
                adj_num = 0;
            }
        }

        is_adj_same || adj_num == 1
    }
}

#[aoc(2019, 4)]
pub fn main() {
    let (start, end) = (347312, 805915);

    // Part I
    let count = (start..=end).into_iter().filter(|x| x.is_valid()).count();
    println!("{count}");

    // Part II
    let count = (start..=end).into_iter().filter(|x| x.is_valid2()).count();
    println!("{count}");
}
