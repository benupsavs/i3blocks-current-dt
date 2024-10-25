use std::{io::{self, BufRead, BufReader, Write}, sync::{mpsc, Arc, Mutex}, thread, time::Duration};

use chrono::{Local, Timelike};
use i3blocks_current_dt::new_timer;

const FORMAT_STR_NORMAL: &str = "%H:%M";
const FORMAT_STR_ALT: &str = "%H %M";
const FORMAT_STR_DATE: &str = "%a %b %-e";

#[derive(PartialEq, Copy, Clone)]
enum DisplayState {
    Clock,
    Date,
    Timer,
}

fn main() -> io::Result<()> {
    let timer_ref_1 = Arc::new(Mutex::new(new_timer()));
    let timer_ref_2 = timer_ref_1.clone();

    let (tx, rx) = mpsc::channel();
    thread::Builder::new().stack_size(16 * 1024).spawn(move || {
        let mut input = BufReader::new(io::stdin());
        let mut line = String::new();
        let mut display_state = DisplayState::Clock;
        loop {
            line.clear();
            if let Ok(_) = input.read_line(&mut line) {
                let mut switch_to = None;
                match line.trim_end() {
                    "3" => { // Right mouse button, switch modes
                        match display_state {
                            DisplayState::Clock => switch_to = Some(DisplayState::Date),
                            DisplayState::Date => switch_to = Some(DisplayState::Timer),
                            DisplayState::Timer => switch_to = Some(DisplayState::Clock),
                        }
                    },
                    "1" => { // Left Mouse Button, start/stop timer
                        if let Ok(mut timer) = timer_ref_1.lock() {
                            timer.toggle();
                            if let Err(e) = tx.send(display_state) {
                                eprintln!("error: {e}");
                            }
                        }

                        switch_to = Some(DisplayState::Timer);
                    },
                    "2" => { // Middle Mouse Button, reset timer
                        if let Ok(mut timer) = timer_ref_1.lock() {
                            timer.reset();
                            if let Err(e) = tx.send(display_state) {
                                eprintln!("error: {e}");
                            }
                        }

                        switch_to = Some(DisplayState::Timer);
                    },
                    _ => {},
                }
                if let Some(switch_to) = switch_to {
                    display_state = switch_to;
                    if let Err(e) = tx.send(switch_to) {
                        eprintln!("error: {e}");
                        break;
                    }
                }
            } else {
                break;
            }
        }
    })?;
    let mut display_state = DisplayState::Clock;
    let mut alt_second = false;
    loop {
        alt_second = !alt_second;
        let now = Local::now();
        let remaining_nanos = 1000000000 - now.nanosecond();
        if let Ok(new_state) = rx.recv_timeout(Duration::new(0, remaining_nanos)) {
            display_state = new_state;
        }

        match display_state {
            DisplayState::Clock => {
                let format_str: &str;
                if alt_second {
                    format_str = FORMAT_STR_ALT;
                } else {
                    format_str = FORMAT_STR_NORMAL;
                }
                println!("{}", now.format(format_str));
            },
            DisplayState::Date => {
                println!("{}", now.format(FORMAT_STR_DATE));
            },
            DisplayState::Timer => {
                if let Ok(timer) = timer_ref_2.lock() {
                    println!("⏲ {}", timer.to_string());
                }
            },
        }
        _ = std::io::stdout().flush();
    }
}
