use std::time::Duration;
use chrono::{Datelike, DateTime, NaiveDateTime, Timelike, TimeZone, Utc};
use humantime::format_duration;

fn main() {
    let duration = Duration::from_secs(9393);
    println!("{}", format_duration(duration));

    let dt = Utc::now();
    let date_time: DateTime<Utc> = Utc.with_ymd_and_hms(dt.year(), dt.month(), dt.day(), dt.hour(), dt.minute(), dt.second()).unwrap();
    let formatted = format!("{}", date_time.format("%d/%m/%Y %H:%M"));
    println!("{}", formatted);
}
