use aoc::cli::{Args, ArgsError};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{fs::File, process::ExitCode};

fn get_template(year: u16, day: u16) -> String {
    format!(
        "use aoc::{{aoc, aoc_input}};

#[aoc({year}, {day})]
pub fn main() {{
    let data = aoc_input!({year}, {day}).unwrap();
}}
"
    )
}

fn create_solution_file(year: u16, day: u16) -> std::io::Result<PathBuf> {
    let tpl = get_template(year, day);
    let location = format!("aoc/src/aoc{year}/day{day}.rs");
    let path = Path::new(&location);

    if path.exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "File already exists",
        ))
    } else {
        let mut output = File::create(path)?;
        write!(output, "{}", tpl)?;
        Ok(path.to_owned())
    }
}

fn update_mod(year: u16, day: u16) -> std::io::Result<Output> {
    let location = format!("aoc/src/aoc{year}/mod.rs");
    let path = Path::new(&location);

    if !path.exists() {
        File::create(path)?;
    }

    let mut output = OpenOptions::new().append(true).open(path)?;
    writeln!(output, "pub mod day{};", day)?;

    Command::new("rustfmt")
        .arg("-q")
        .arg(path.as_os_str())
        .output()
}

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

    if args.day.is_none() {
        eprintln!("generating all inputs for a given year is not supported")
    }

    let day = args.day.unwrap();
    let year = args.year;

    match create_solution_file(year, day) {
        Ok(path) => {
            println!("Created a file: {path:?}");
        }
        Err(e) => {
            eprintln!("Could not create a file: {e}");
            return ExitCode::FAILURE;
        }
    }
    match update_mod(year, day) {
        Ok(_) => {
            println!("Updated mod.rs");
            ExitCode::SUCCESS
        }
        Err(_) => {
            eprintln!("Could not update mod.rs");
            ExitCode::FAILURE
        }
    }
}
