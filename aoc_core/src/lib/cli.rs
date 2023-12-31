use crate::date::get_current_year;

pub const HELP: &str = "\
USAGE: {} [FLAGS] [OPTIONS] -y <YEAR>

FLAGS:
  -h, --help    Prints help information

OPTIONS:
  -y YEAR       Sets AoC year; use current year if not provided
  -d DAY        Sets AoC day; if not present - iterate over 1..=25
";

fn prog() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_string()
        .into()
}

#[derive(Debug, Clone)]
pub enum ArgsError {
    Help,
    InvalidDay,
    InvalidYear,
    Error(String),
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => {
                let help = HELP.replace("{}", &prog().unwrap_or("aoc".to_string()));
                write!(f, "{}", help)
            }
            Self::InvalidDay => write!(f, "Valid days are: 1, 2, ..., 25"),
            Self::InvalidYear => write!(f, "AoC started in 2015"),
            Self::Error(msg) => write!(f, "Error while parsing arguments: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub day: Option<u16>,
    pub year: u16,
}

impl TryFrom<pico_args::Arguments> for Args {
    type Error = ArgsError;

    fn try_from(mut args: pico_args::Arguments) -> Result<Self, Self::Error> {
        if args.contains(["-h", "--help"]) {
            return Err(ArgsError::Help);
        }

        let day = args
            .opt_value_from_str("-d")
            .map_err(|e| ArgsError::Error(format!("{e}")))?;
        let year = args
            .value_from_str("-y")
            .unwrap_or_else(|_| get_current_year());

        if let Some(d) = day {
            if !(0..=25).contains(&(d as i32)) {
                return Err(ArgsError::InvalidDay);
            }
        }
        if year < 2015 {
            return Err(ArgsError::InvalidYear);
        }

        Ok(Args { day, year })
    }
}
