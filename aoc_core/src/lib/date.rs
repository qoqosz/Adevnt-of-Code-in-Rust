use std::time::{SystemTime, UNIX_EPOCH};

/// Get the current year
/// Based on: http://howardhinnant.github.io/date_algorithms.html#civil_from_days
pub fn get_current_year() -> u16 {
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let z = timestamp / 86400 + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;

    (yoe + era * 400) as u16
}
