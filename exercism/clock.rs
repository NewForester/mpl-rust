//! Exercism Rust Track:  a solution for the clock exercise

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    time: i16,
}

impl Clock {
    /// Function new() allows an inititialised clock to be created
    pub fn new (hh: i16, mm: i16) -> Clock {
        let mut time = (hh * 60 + mm) % (24 * 60);

        if time < 0 {time += 24 * 60;}

        Clock {time}
    }

    /// Function add_minutes() allows the clock time to be adjusted
    pub fn add_minutes(self, mm: i16) -> Clock {
        Clock::new(0, self.time + mm)
    }
}

impl std::string::ToString for Clock {
    /// Function to_string() formats the clock time for printing
    fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.time / 60, self.time % 60)
    }
}
