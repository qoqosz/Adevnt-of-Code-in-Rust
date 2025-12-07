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
        let year = $crate::date::get_current_year();
        aoc_input!(year, $d)
    };

    ($y:expr, $d:expr) => {{
        $crate::io::get_from_cache($crate::io::AOC_DIR, $y as u16, $d as u16).or_else(|_| {
            let session_cookie =
                $crate::io::load_cookie($crate::io::AOC_ENV_VAR, $crate::io::AOC_COOKIE_FILE).ok();
            $crate::io::create_store($crate::io::AOC_DIR)?;

            if let Ok(session_cookie) =
                $crate::io::load_cookie($crate::io::AOC_ENV_VAR, $crate::io::AOC_COOKIE_FILE)
            {
                let puzzle_input =
                    $crate::io::download_aoc_input(&session_cookie, $y as u16, $d as u16)?;
                $crate::io::save_to_cache(
                    $crate::io::AOC_DIR,
                    $y as u16,
                    $d as u16,
                    &puzzle_input,
                )?;
                Ok(puzzle_input)
            } else {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Empty AoC session cookie",
                ));
            }
        })
    }};
}
