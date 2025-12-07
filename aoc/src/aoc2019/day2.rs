use aoc::{aoc, aoc_input};

fn eval(input: &mut [usize]) -> usize {
    let mut i = 0;

    loop {
        let op = input[i];

        if op == 99 {
            break;
        }

        let (j, k, m) = (input[i + 1], input[i + 2], input[i + 3]);
        input[m] = match op {
            1 => input[j] + input[k],
            2 => input[j] * input[k],
            _ => unreachable!("invalid op code"),
        };

        i += 4;
    }

    input[0]
}

fn find(input: &[usize], solution: usize) -> Option<(usize, usize)> {
    for a in 0..99 {
        for b in 0..99 {
            let mut tmp = input.to_vec();
            tmp[1] = a;
            tmp[2] = b;

            if eval(&mut tmp) == solution {
                return Some((a, b));
            }
        }
    }
    None
}

#[aoc(2019, 2)]
pub fn main() {
    let data = aoc_input!(2019, 2).unwrap();
    let input = data
        .split(',')
        .flat_map(|x| x.parse::<usize>())
        .collect::<Vec<_>>();

    // Part I
    let mut input1 = input.clone();
    input1[1] = 12;
    input1[2] = 2;
    println!("{}", eval(&mut input1));

    // Part II
    let output = 19690720;
    let (noun, verb) = find(&input, output).unwrap();
    println!("{:?}", 100 * noun + verb);
}
