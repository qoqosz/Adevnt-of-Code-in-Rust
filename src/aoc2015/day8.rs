fn main() {
    let data = vec!["\"\"", "\"abc\"", "\"aaa\\\"aaa\"", "0x27"];

    for x in data.iter() {
        println!("{}", x.chars().count());
    }
}
