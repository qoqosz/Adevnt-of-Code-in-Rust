use aoc::cli::{Args, ArgsError};
use std::io::Write;
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

fn create_file(year: u16, day: u16) -> std::io::Result<()> {
    let tpl = get_template(year, day);
    let path = format!("aoc/src/aoc{year}/day{day}.rs");
    let mut output = File::create(path)?;
    write!(output, "{}", tpl)
}

// TODO: This is a quick draft, needs polishing
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

    match create_file(year, day) {
        Ok(()) => {
            println!("Created a file");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("Could not create a file: {e}");
            ExitCode::FAILURE
        }
    }
}
