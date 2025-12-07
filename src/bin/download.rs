// TODO: File doc
use aoc::{aoc_input, date::get_current_year};

const HELP: &str = "\
USAGE: download [FLAGS] [OPTIONS] -d <DAY>

FLAGS:
  -h, --help    Prints help information

OPTIONS:
  -y YEAR       Sets AoC year; use current year if not provided
  -d DAY        Sets AoC day
";

#[derive(Debug, Clone)]
enum ArgsError {
    Help,
    Error(String),
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => write!(f, "{}", HELP),
            Self::Error(msg) => write!(f, "Error while parsing arguments: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
struct Args {
    pub day: u16,
    pub year: u16,
}

impl TryFrom<pico_args::Arguments> for Args {
    type Error = ArgsError;

    fn try_from(mut args: pico_args::Arguments) -> Result<Self, Self::Error> {
        if args.contains(["-h", "--help"]) {
            return Err(ArgsError::Help);
        }

        let day = match args.value_from_str("-d") {
            Ok(day) => day,
            Err(e) => {
                return Err(ArgsError::Error(format!("{e}")));
            }
        };

        Ok(Args {
            day,
            year: args
                .value_from_str("-y")
                .unwrap_or_else(|_| get_current_year()),
        })
    }
}

fn main() -> std::process::ExitCode {
    let pargs = pico_args::Arguments::from_env();
    let args = match Args::try_from(pargs) {
        Ok(args) => args,
        Err(help @ ArgsError::Help) => {
            eprintln!("{help}");
            return std::process::ExitCode::SUCCESS;
        }
        Err(err) => {
            eprintln!("{err}");
            return std::process::ExitCode::FAILURE;
        }
    };

    if let Err(err) = aoc_input!(args.year, args.day) {
        eprintln!("{}", err);
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
