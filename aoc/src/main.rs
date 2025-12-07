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

    if let Some(f) =
        inventory::iter::<Solution>().find(|f| (f.year, f.day) == (args.year, args.day))
    {
        f.run();
    } else {
        unimplemented!()
    }

    ExitCode::SUCCESS
}
