//! Exercism Rust Track:  a solution for the clock exercise

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hh: i16,
    mm: i16,
}

impl Clock {
    /// Function new() allows an inititialised clock to be created
    pub fn new (hh: i16, mm: i16) -> Clock {

        let mut clock = Clock {hh, mm};

        clock.normalise();

        clock
    }

    /// Function add_minutes() allows the clock time to be adjusted
    pub fn add_minutes(mut self, mm: i16) -> Clock {

        self.mm += mm;

        self.normalise();

        self
    }

    /// Function normalise() ensures the internal representation is suitable for formatting and comparison
    fn normalise(&mut self) -> () {
        let time = self.hh * 60 + self.mm;

        self.hh = time / 60 % 24;
        self.mm = time % 60;

        if self.mm < 0 {
            self.hh -= 1;
            self.mm += 60;
        }
        if self.hh < 0 {
            self.hh += 24;
        }
    }
}

impl std::string::ToString for Clock {
    /// Function to_string() formats the clock time for printing
    fn to_string(&self) -> String {

        format!("{:02}:{:02}", self.hh, self.mm)
    }
}

//
// This solution follows the form of the others examined.  There is little
// choice.
//
// Half stored the time as a large minute value, half as hour and minutes.
// It makes little difference.
//
// All stored normalised values, several had separate normalise routines.
//
// All implemented ToString but only one implemented PartialEq.
//
// Mine was unique in returning a modified Clock in the Add routine rather
// than a new Clock.  Probably mine is incorrect.
//
// If I do this again I suggest return a new clock from Add.  Then normalise
// need not be a separate routine.  Changing the structure to hold just
// minutes should enable the normalisation code to be shortened by using
// 24 * 60.
//
