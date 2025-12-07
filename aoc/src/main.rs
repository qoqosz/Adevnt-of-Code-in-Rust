pub mod aoc2015;
pub mod aoc2016;
pub mod aoc2020;
pub mod aoc2021;
pub mod aoc2023;

use aoc::cli::{Args, ArgsError};
use aoc_core::Solution;
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

    if args.day != 0 {
        if let Some(solution) =
            inventory::iter::<Solution>().find(|sol| **sol == (args.year, args.day))
        {
            solution.run();
        } else {
            unimplemented!()
        }
    } else {
        for solution in
            inventory::iter::<Solution>().filter(|sol| sol.year == args.year && sol.day < 18)
        {
            solution.run();
        }
    }

    ExitCode::SUCCESS
}
