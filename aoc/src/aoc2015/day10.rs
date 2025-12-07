use aoc::aoc;

fn look_and_say(text: &str) -> String {
    let mut out: Vec<String> = Vec::with_capacity(text.len() * 2);
    let mut count = 0;
    let mut current = text.chars().next().unwrap();

    for ch in text.chars() {
        if ch == current {
            count += 1
        } else {
            out.push(format!("{}{}", count, current));
            count = 1;
            current = ch;
        }
    }
    out.push(format!("{}{}", count, current));
    out.join("")
}

fn play(text: &str, n: usize) -> usize {
    (0..n)
        .fold(text.to_owned(), |txt, _| look_and_say(&txt))
        .len()
}

#[aoc(2015, 10)]
pub fn main() {
    let data = "1321131112";

    // Part I
    println!("{}", play(data, 40));

    // Part II
    println!("{}", play(data, 50));
}
