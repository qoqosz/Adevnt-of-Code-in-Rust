fn look_and_say(text: &String) -> String {
    let mut out: Vec<String> = Vec::new();
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

fn play(text: &String, n: usize) -> usize {
    let mut text = text.clone();

    for i in 0..n {
        text = look_and_say(&text);
    }
    text.len()
}

fn main() {
    let data = "1321131112".to_string();

    // Part I
    println!("{}", play(&data, 40));

    // Part II
    println!("{}", play(&data, 50));
}
