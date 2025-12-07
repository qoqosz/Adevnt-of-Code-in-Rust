/// Load input data either from a given path or from the 1<sup>st</sup> cmd line arg.
#[macro_export]
macro_rules! load_input {
    () => {
        match std::env::args().nth(1) {
            Some(fname) => load_input!(fname),
            _ => {
                eprint!("No input specified!");
                std::process::exit(1);
            }
        }
    };

    ($p:expr) => {
        std::fs::read_to_string($p).expect("Unable to read input")
    };
}

/// Load AoC puzzle input
///
/// This uses local cache to store once downloaded inputs. Session cookie is required to download new files.
#[macro_export]
macro_rules! aoc_input {
    ($d:expr) => {
        let year = chrono::Utc::now().year();
        aoc_input!(year, $d)
    };

    ($y:expr, $d:expr) => {{
        let session_cookie =
            $crate::io::load_cookie($crate::io::AOC_ENV_VAR, $crate::io::AOC_COOKIE_FILE)
                .expect("No AoC session cookie found");
        $crate::io::get_aoc_input($crate::io::AOC_DIR, &session_cookie, $y as u16, $d as u16)
    }};
}
