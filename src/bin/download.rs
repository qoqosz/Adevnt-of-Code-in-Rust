/// TODO: File doc
use aoc::aoc_input;
use clap::{arg, Command};

fn main() {
    let matches = Command::new("Advent of Code - puzzle input downloader")
        .version("0.1.0")
        .arg(
            arg!(-y --year <YEAR> "Year")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            arg!(-d --day <DAY> "Day")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .get_matches();

    let year = *matches.get_one::<u16>("year").expect("required");
    let day = *matches.get_one::<u16>("day").expect("required");
    let _ = aoc_input!(year, day);
}
