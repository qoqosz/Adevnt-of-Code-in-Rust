// pub mod aoc2015;
// pub mod aoc2016;
// pub mod aoc2020;
// pub mod aoc2021;
pub mod aoc2022;
// pub mod aoc2023;

use aoc::cli::{Args, ArgsError};
use aoc_core::solution::Solution;
use itertools::Itertools;
use std::process::ExitCode;

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

    for solution in inventory::iter::<Solution>()
        .filter(|sol| {
            let is_year = sol.year == args.year;
            let is_day = match args.day {
                Some(d) => sol.day == d,
                _ => true,
            };
            is_day && is_year
        })
        .collect::<Vec<_>>()
        .iter()
        .sorted_by_key(|sol| (sol.year, sol.day))
    {
        println!("Day {}, {}", solution.day, solution.year);
        solution.run();
        println!();
    }

    ExitCode::SUCCESS
}
