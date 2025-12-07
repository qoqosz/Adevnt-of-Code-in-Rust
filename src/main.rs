mod aoc2015;
mod aoc2016;
mod aoc2020;
mod aoc2021;
mod aoc2023;

use aoc::cli::{Args, ArgsError};
use std::process::ExitCode;
use std::time::Instant;

fn main() -> ExitCode {
    let pargs = pico_args::Arguments::from_env();
    let args = match Args::try_from(pargs) {
        Ok(args) => args,
        Err(help @ ArgsError::Help) => {
            eprintln!("{help}");
            return ExitCode::SUCCESS;
        }
        Err(err) => {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    };

    let time = Instant::now();

    match (args.year, args.day) {
        // 2015
        (2015, 1) => aoc2015::day1::main(),
        (2015, 2) => aoc2015::day2::main(),
        (2015, 3) => aoc2015::day3::main(),
        (2015, 4) => aoc2015::day4::main(),
        (2015, 5) => aoc2015::day5::main(),
        (2015, 6) => aoc2015::day6::main(),
        (2015, 7) => aoc2015::day7::main(),
        (2015, 8) => aoc2015::day8::main(),
        (2015, 9) => aoc2015::day9::main(),
        (2015, 10) => aoc2015::day10::main(),
        (2015, 11) => aoc2015::day11::main(),
        (2015, 12) => aoc2015::day12::main(),
        (2015, 13) => aoc2015::day13::main(),
        (2015, 14) => aoc2015::day14::main(),
        (2015, 15) => aoc2015::day15::main(),
        (2015, 16) => aoc2015::day16::main(),
        (2015, 17) => aoc2015::day17::main(),
        (2015, 18) => aoc2015::day18::main(),
        (2015, 19) => aoc2015::day19::main(),
        (2015, 20) => aoc2015::day20::main(),
        (2015, 21) => aoc2015::day21::main(),
        (2015, 22) => aoc2015::day22::main(),
        (2015, 23) => aoc2015::day23::main(),
        (2015, 24) => aoc2015::day24::main(),
        (2015, 25) => aoc2015::day25::main(),
        // 2016
        (2016, 1) => aoc2016::day1::main(),
        // 2020
        (2020, 1) => aoc2020::day1::main(),
        (2020, 2) => aoc2020::day2::main(),
        (2020, 3) => aoc2020::day3::main(),
        (2020, 4) => aoc2020::day4::main(),
        (2020, 5) => aoc2020::day5::main(),
        (2020, 6) => aoc2020::day6::main(),
        (2020, 7) => aoc2020::day7::main(),
        (2020, 8) => aoc2020::day8::main(),
        (2020, 9) => aoc2020::day9::main(),
        (2020, 10) => aoc2020::day10::main(),
        (2020, 11) => aoc2020::day11::main(),
        (2020, 13) => aoc2020::day13::main(),
        // 2021
        (2021, 15) => aoc2021::day15::main(),
        // 2023
        (2023, 2) => aoc2023::day1::main(),
        _ => {
            unimplemented!()
        }
    };

    let duration = time.elapsed().as_micros() as f64 / 1000.0;

    println!("Elapsed: {duration:0.2} ms");

    ExitCode::SUCCESS
}
