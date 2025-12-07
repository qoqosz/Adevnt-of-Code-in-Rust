use crate::date::get_current_year;

pub const HELP: &str = "\
USAGE: {} [FLAGS] [OPTIONS] -d <DAY>

FLAGS:
  -h, --help    Prints help information

OPTIONS:
  -y YEAR       Sets AoC year; use current year if not provided
  -d DAY        Sets AoC day
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
    Error(String),
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Help => {
                let help = HELP.replace("{}", &prog().unwrap());
                write!(f, "{}", help)
            }
            Self::Error(msg) => write!(f, "Error while parsing arguments: {}", msg),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub day: u16,
    pub year: u16,
}

impl TryFrom<pico_args::Arguments> for Args {
    type Error = ArgsError;

    fn try_from(mut args: pico_args::Arguments) -> Result<Self, Self::Error> {
        if args.contains(["-h", "--help"]) {
            return Err(ArgsError::Help);
        }

        Ok(Args {
            day: args
                .value_from_str("-d")
                .map_err(|e| ArgsError::Error(format!("{e}")))?,
            year: args
                .value_from_str("-y")
                .unwrap_or_else(|_| get_current_year()),
        })
    }
}
