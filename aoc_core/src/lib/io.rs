use crate::num::Unsigned;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

extern crate shellexpand;

pub static AOC_DIR: &str = "~/.aoc";
pub static AOC_ENV_VAR: &str = "AOC_SESSION";
pub static AOC_COOKIE_FILE: &str = "session.cookie";

/// Get a path to a file with puzzle input.
fn get_path(cache_dir: impl AsRef<str>, year: impl Unsigned, day: impl Unsigned) -> PathBuf {
    let cache_dir: &str = cache_dir.as_ref();
    let cache_dir = shellexpand::tilde::<&str>(&cache_dir);

    Path::new(cache_dir.as_ref())
        .join(format!("{}", year))
        .join(format!("{}.txt", day))
}

/// Create a directory if it does not exist.
fn create_store(path: impl AsRef<str>) -> io::Result<()> {
    let path: &str = path.as_ref();
    let path = shellexpand::tilde::<&str>(&path);

    fs::create_dir_all(path.as_ref())?;
    Ok(())
}

/// Read puzzle input from a cache dir if it does exist.
fn get_from_cache(
    cache_dir: impl AsRef<str>,
    year: impl Unsigned,
    day: impl Unsigned,
) -> io::Result<String> {
    let path = get_path(cache_dir, year, day);
    fs::read_to_string(path)
}

/// Write puzzle input to a file in cache dir.
fn save_to_cache(
    cache_dir: impl AsRef<str>,
    year: impl Unsigned,
    day: impl Unsigned,
    input: &String,
) -> io::Result<()> {
    let path = get_path(cache_dir, year, day);
    let prefix = path.parent().unwrap();
    fs::create_dir_all(prefix)?;
    fs::write(path, input)
}

/// Load cookie from an env var or a file
pub fn load_cookie(env_var: &str, cookie_file: &str) -> io::Result<String> {
    let trim = |x: &String| x.trim().to_string();
    match std::env::var(env_var) {
        Ok(ref var) if !trim(var).is_empty() => Ok(trim(var)),
        _ => {
            let cwd = std::env::current_dir()?;
            fs::read_to_string(cwd.join(cookie_file)).map(|x| trim(&x))
        }
    }
}

/// Use a client (which holds a cookie) to download the input from AoC website
fn download_file(url: &str, session_cookie: &str) -> Result<String, io::Error> {
    let response = match ureq::get(url)
        .set("cookie", session_cookie)
        .set(
            "User-Agent",
            "https://github.com/qoqosz/Adevnt-of-Code-in-Rust by qoqosz@gmail.com",
        )
        .call()
    {
        Ok(resp) => resp,
        Err(_) => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Problem downloading the input",
            ))
        }
    };
    let body = response.into_string().unwrap_or_default();
    Ok(body)
}

/// Download AoC puzzle input for a given year and day.
fn download_aoc_input(
    session_cookie: &str,
    year: impl Unsigned,
    day: impl Unsigned,
) -> Option<String> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let cookie = format!("session={}", session_cookie.trim());

    download_file(&url, &cookie).ok()
}

pub fn get_aoc_input(
    cache_dir: impl AsRef<str>,
    session_cookie: Option<&str>,
    year: impl Unsigned,
    day: impl Unsigned,
) -> io::Result<String> {
    create_store(&cache_dir)?;

    get_from_cache(&cache_dir, year, day).or_else(|_| {
        if session_cookie.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Empty AoC session cookie",
            ));
        }
        let puzzle_input = download_aoc_input(session_cookie.unwrap(), year, day);

        match puzzle_input {
            Some(input) => {
                save_to_cache(cache_dir, year, day, &input)?;
                Ok(input)
            }
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Error retrieving AoC input",
            )),
        }
    })
}
