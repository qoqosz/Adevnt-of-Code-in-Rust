use aoc::{aoc, aoc_input};

fn advance(state: &Vec<usize>) -> Vec<usize> {
    let mut new_state = vec![0; 9];

    for (i, &cnt) in state.iter().enumerate() {
        if i == 0 {
            new_state[6] += cnt;
            new_state[8] += cnt;
        } else {
            new_state[i - 1] += cnt;
        }
    }

    new_state
}

fn simulate(state: &Vec<usize>, n: usize) -> usize {
    let mut state = state.clone();

    for _ in 0..n {
        state = advance(&state);
    }

    state.iter().sum::<usize>()
}

fn parse(line: &str) -> Vec<usize> {
    let mut state = vec![0; 9];

    for x in line.trim().split(',').flat_map(|x| x.parse::<usize>()) {
        state[x] += 1;
    }

    state
}

#[aoc(2021, 6)]
pub fn main() {
    let data = aoc_input!(2021, 6).unwrap();
    let state = parse(&data);

    // Part I
    println!("{}", simulate(&state, 80));

    // Part II
    println!("{}", simulate(&state, 256));
}
