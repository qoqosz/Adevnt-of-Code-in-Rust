use aoc::{aoc, aoc_input};

#[memoize::memoize]
fn count_stones(num: usize, n_blinks: usize) -> usize {
    if n_blinks == 0 {
        return 1;
    }
    if num == 0 {
        return count_stones(1, n_blinks - 1);
    }

    let num_len = (num.ilog10() + 1) as usize;

    if num_len % 2 == 0 {
        let num_str = num.to_string();
        let (left, right) = num_str.split_at(num_len / 2);
        let (left, right) = (
            left.parse::<usize>().unwrap(),
            right.parse::<usize>().unwrap(),
        );
        return count_stones(left, n_blinks - 1) + count_stones(right, n_blinks - 1);
    } else {
        return count_stones(num * 2024, n_blinks - 1);
    }
}

#[aoc(2024, 11)]
pub fn main() {
    let data = aoc_input!(2024, 11).unwrap();
    let stones: Vec<_> = data
        .trim()
        .split(' ')
        .flat_map(|x| x.parse::<usize>())
        .collect();

    let solve = |n| stones.iter().map(|s| count_stones(*s, n)).sum::<usize>();

    // Part I
    println!("{}", solve(25));

    // Part II
    println!("{}", solve(75));
}
