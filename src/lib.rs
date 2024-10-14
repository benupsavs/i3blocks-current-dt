use std::time::{Duration, Instant};

#[derive(Default)]
pub struct Timer {
    additional_time: Duration,
    start_time: Option<Instant>,
}

/// Creates a new timer.
pub fn new_timer() -> Timer {
    Timer{..Default::default()}
}

impl Timer {
    pub fn toggle(&mut self) {
        match self.start_time {
            Some(start_time) => {
                _ = self.start_time.take();
                let now = Instant::now();
                self.additional_time += now.duration_since(start_time);
            },
            None => self.start_time = Some(Instant::now()),
        }
    }

    pub fn reset(&mut self) {
        self.additional_time = Duration::default();
        self.start_time = None;
    }
}

impl ToString for Timer {
    fn to_string(&self) -> String {
        let mut duration = self.additional_time.clone();
        if let Some(st) = self.start_time {
            duration += Instant::now().duration_since(st);
        }

        let mut hours = 0;
        let mut minutes = 0;
        let mut seconds = duration.as_secs();
        while seconds >= 60 {
            seconds -= 60;
            if minutes == 59 {
                minutes = 0;
                hours += 1;
            } else {
                minutes += 1;
            }
        }

        return if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{:02}:{:02}", minutes, seconds)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_string_one_second_stopped() {
        let timer = Timer{
            additional_time: Duration::from_secs(1),
            start_time: None,
        };

        assert_eq!("00:01", timer.to_string());
    }

    #[test]
    fn test_to_string_one_second_running() {
        let timer = Timer{
            additional_time: Duration::default(),
            start_time: Some(Instant::now() - Duration::from_secs(1)),
        };

        assert_eq!("00:01", timer.to_string());
    }
}
