use crate::num::Unsigned;
use reqwest::{header, Client, ClientBuilder};
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
) -> Option<String> {
    let path = get_path(cache_dir, year, day);
    fs::read_to_string(path).ok()
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
pub fn load_cookie(env_var: &str, cookie_file: &str) -> Option<String> {
    let trim = |x: String| x.trim().to_string();
    match std::env::var(env_var) {
        Ok(var) => Some(trim(var)),
        _ => fs::read_to_string(cookie_file).map(trim).ok(),
    }
}

/// Use a client (which holds a cookie) to download the input from AoC website
async fn download_file(client: &Client, url: &str) -> reqwest::Result<String> {
    let response = client.get(url).send().await?;
    let data = response.text().await?;
    Ok(data)
}

/// Download AoC puzzle input for a given year and day.
async fn download_aoc_input(
    session_cookie: &str,
    year: impl Unsigned,
    day: impl Unsigned,
) -> Option<String> {
    let mut request_headers = header::HeaderMap::new();
    request_headers.insert(
        header::COOKIE,
        header::HeaderValue::from_str(&format!("session={}", session_cookie.trim())).unwrap(),
    );
    let client = ClientBuilder::new()
        .default_headers(request_headers)
        .build()
        .unwrap();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    download_file(&client, &url).await.ok()
}

async fn aget_aoc_input(
    cache_dir: impl AsRef<str>,
    session_cookie: &str,
    year: impl Unsigned,
    day: impl Unsigned,
) -> io::Result<String> {
    create_store(&cache_dir)?;

    match get_from_cache(&cache_dir, year, day) {
        Some(input) => Ok(input),
        _ => {
            let puzzle_input = download_aoc_input(session_cookie, year, day)
                .await
                .map(|input| {
                    save_to_cache(cache_dir, year, day, &input).unwrap_or(());
                    input
                });

            puzzle_input.ok_or(io::Error::new(
                io::ErrorKind::Other,
                "Error retrieving AoC input",
            ))
        }
    }
}

pub fn get_aoc_input(
    cache_dir: impl AsRef<str>,
    session_cookie: &str,
    year: impl Unsigned,
    day: impl Unsigned,
) -> io::Result<String> {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(aget_aoc_input(cache_dir, session_cookie, year, day))
}
