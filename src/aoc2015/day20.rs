/// Use a sieve to fill in `taregt` houses with presents from elves
fn solve(target: usize, limit: usize) -> Option<usize> {
    let mut houses: Vec<usize> = vec![1; target];

    for elf in 2..target {
        for visit in 0..std::cmp::min(target / elf, limit) {
            houses[elf * visit] += elf;
        }
        if houses[elf] >= target {
            return Some(elf);
        }
    }
    None
}

fn main() {
    let input: usize = 33100000;

    // Part I
    println!("{}", solve(input / 10, input / 10).unwrap());

    // Part II
    println!("{}", solve(input / 11, 50).unwrap());
}
