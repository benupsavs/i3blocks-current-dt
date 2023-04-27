use std::{thread, time::Duration, io::{Write, BufReader, self, BufRead}, sync::mpsc};

use chrono::{Local, Timelike};

const FORMAT_STR_NORMAL: &str = "%a %b %-e %H:%M:%S";
const FORMAT_STR_ALT: &str = "%Y-%m-%d";

fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::Builder::new().stack_size(16 * 1024).spawn(move || {
        let mut input = BufReader::new(io::stdin());
        let mut line = String::new();
        let mut alt_state = false;
        loop {
            line.clear();
            if let Ok(_) = input.read_line(&mut line) {
                if line.trim_end() == "1" {
                    alt_state = !alt_state;
                    if let Err(e) = tx.send(alt_state) {
                        eprintln!("error: {e}");
                        break;
                    }
                }
            } else {
                break;
            }
        }
    })?;
    let mut alt_state = false;
    loop {
        let now = Local::now();
        let remaining_nanos = 1000000000 - now.nanosecond();
        if let Ok(new_state) = rx.recv_timeout(Duration::new(0, remaining_nanos)) {
            alt_state = new_state;
        }

        let format_str: &str;
        if alt_state {
            format_str = FORMAT_STR_ALT;
        } else {
            format_str = FORMAT_STR_NORMAL;
        }
        println!("{}", now.format(format_str));
        _ = std::io::stdout().flush();
    }
}
