use std::time::Duration;
use humantime::format_duration;

fn main() {
    let duration = Duration::from_secs(9393);
    println!("{}", format_duration(duration));
}
