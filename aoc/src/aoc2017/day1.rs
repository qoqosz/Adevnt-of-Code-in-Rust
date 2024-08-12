use aoc::{aoc, aoc_input};

fn solve_captcha(data: &str, shift: usize) -> usize {
    let line = data.lines().next().unwrap().as_bytes();
    let n = line.len();
    let mut sum = 0usize;

    for i in 0..n {
        let (x, y) = (line[i], line[(i + shift) % n]);
        if x == y {
            sum += (x - 48) as usize;
        }
    }
    sum
}

#[aoc(2017, 1)]
pub fn main() {
    let data = aoc_input!(2017, 1).unwrap();

    // Part I
    let sum = solve_captcha(&data, 1);
    println!("{sum}");

    // Part II
    let n = data.find('\n').unwrap();
    let sum = solve_captcha(&data, n / 2);
    println!("{sum}");
}

#[cfg(test)]
mod test {

    use super::solve_captcha;

    #[test]
    fn test_part1() {
        let data = vec!["1122", "1111", "1234", "91212129"];
        let expected = vec![3, 4, 0, 9];

        for (txt, count) in data.iter().zip(expected.into_iter()) {
            assert_eq!(solve_captcha(txt, 1), count);
        }
    }

    #[test]
    fn test_part2() {
        let data = vec!["1212", "1221", "123425", "123123", "12131415"];
        let expected = vec![6, 0, 4, 12, 4];

        for (txt, count) in data.iter().zip(expected.into_iter()) {
            let n = txt.len();
            assert_eq!(solve_captcha(txt, n / 2), count);
        }
    }
}
