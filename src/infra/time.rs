use chrono::{DateTime, Datelike, TimeZone, Timelike};
use std::fmt::Display;

// HACK:
// Sometimes cnblogs' web API returns time string like: "2023-09-12T14:07:00" or "2019-02-06T08:45:53.94"
// This will patch it to standard RFC3339 format
pub fn patch_rfc3339(time_str: &str) -> String {
    if time_str.len() != 25 {
        let u8vec: Vec<_> = time_str.bytes().take(19).collect();
        format!(
            "{}+08:00",
            String::from_utf8(u8vec)
                .unwrap_or_else(|_| panic!("Can not patch time string: {}", time_str))
        )
    } else {
        time_str.to_owned()
    }
}

pub fn fmt_time_to_string_friendly<T>(time_to_fmt: DateTime<T>, current_time: DateTime<T>) -> String
where
    T: TimeZone,
    <T as TimeZone>::Offset: Display,
{
    let diff = current_time.clone() - time_to_fmt.clone();
    match diff {
        // In the future
        _ if diff.num_milliseconds() < 0 => time_to_fmt.format("%Y-%m-%d %H:%M").to_string(),
        // Same year...
        _ if time_to_fmt.year() != current_time.year() => {
            time_to_fmt.format("%Y-%m-%d").to_string()
        }
        _ if time_to_fmt.month() != current_time.month() => {
            time_to_fmt.format("%m-%d %H:%M").to_string()
        }
        _ if time_to_fmt.day() != current_time.day() => {
            let postfix = match time_to_fmt.day() {
                1 => "st",
                2 => "nd",
                3 => "rd",
                _ => "th",
            };
            time_to_fmt
                .format(&format!("%d{} %H:%M", postfix))
                .to_string()
        }
        _ if time_to_fmt.hour() != current_time.hour() => time_to_fmt.format("%H:%M").to_string(),
        // Within an hour
        _ if diff.num_seconds() < 30 => "Now".to_owned(),
        _ if diff.num_minutes() < 3 => "Recently".to_owned(),
        _ if diff.num_minutes() < 30 => format!("{}m", diff.num_minutes()),
        _ => time_to_fmt.format("%H:%M").to_string(),
    }
}
