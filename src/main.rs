use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use chrono::Utc;
use chrono_tz::America::New_York;
use chrono_tz::Asia::Seoul;

fn get_curent_time_est() -> String {
    let utc = Utc::now();
    let ny_time = utc.with_timezone(&New_York);
    return format!(" {}", ny_time);
}

fn get_curent_time_seoul() -> String {
    let utc = Utc::now();
    let kor_time = utc.with_timezone(&Seoul);
    return format!(" {}", kor_time);
}

fn main() {
    loop {
        print!("New_York Time: {}\r", get_curent_time_est());
        print!("\n_Korea__ Time: {}\r", get_curent_time_seoul());
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(1000));
    }
}
