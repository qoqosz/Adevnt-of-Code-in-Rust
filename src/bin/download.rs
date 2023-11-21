// TODO: File doc
use aoc::{aoc_input, date::get_current_year};
use clap::{arg, Command};

fn main() -> std::process::ExitCode {
    let matches = Command::new("Advent of Code - puzzle input downloader")
        .version("0.1.0")
        .arg(
            arg!(-y --year <YEAR> "Year, defaults to current year")
                .required(false)
                .value_parser(clap::value_parser!(u16)),
        )
        .arg(
            arg!(-d --day <DAY> "Day")
                .required(true)
                .value_parser(clap::value_parser!(u16)),
        )
        .get_matches();

    let year = *matches
        .get_one::<u16>("year")
        .unwrap_or(&get_current_year());
    let day = *matches.get_one::<u16>("day").expect("required");

    if let Err(e) = aoc_input!(year, day) {
        eprintln!("{}", e);
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
