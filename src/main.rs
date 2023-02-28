use std::{thread, time::Duration, io::Write};

use chrono::{Local, Timelike};


fn main() {
    loop {
        let now = Local::now();
        println!("{}", now.format("%a %b %e %H:%M:%S"));
        _ = std::io::stdout().flush();
        let sleep_nanos = 1000000000 - now.nanosecond();
        thread::sleep(Duration::new(0, sleep_nanos));
    }
}
