// TODO: File doc
use aoc::aoc_input;
use aoc::cli::{Args, ArgsError};
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

    if let Some(day) = args.day {
        if let Err(err) = aoc_input!(args.year, day) {
            eprintln!("{err}");
            return ExitCode::FAILURE;
        }
    } else {
        eprintln!("downloading all inputs for a given year is not supported")
    }
    ExitCode::SUCCESS
}
