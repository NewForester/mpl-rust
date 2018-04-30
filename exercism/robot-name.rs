//! Exercism Rust Track:  a solution for the robot-name exercise

extern crate rand;

use rand::Rng;

/// Macro makeletter!() takes a natural number and returns an uppercase ASCII letter as a character
macro_rules! makeletter {
    ($number:expr) => (
        (($number % 26) + ('A' as u32)) as u8 as char;
    )
}

/// The Robot structure has only a name but the name is unique
pub struct Robot {
    name: String
}

/// The Robot implementation as required by the exercise's test cases
impl Robot {
    /// new() returns a new Robot with a unique name
    pub fn new() -> Robot
    {
        Robot {
            name:  Self::new_name(),
        }
    }

    /// name() returns the Robot's unique name
    pub fn name<'a>(&'a self) -> &'a str
    {
        &self.name[..]
    }

    /// reset_name() gives the Robot a new unique name
    pub fn reset_name(&mut self) {
        self.name = Self::new_name()
    }

    /// new_name() returns a suitably unique name for a Robot
    fn new_name() -> String {
        let rn: u32 = rand::thread_rng().gen_range(0, 26*26*1000);

        format!("{}{}{:03}", makeletter!(rn / 1000 / 26), makeletter!(rn / 1000), rn % 1000)
    }
}

 //
// There was a lot of variation so I guess others had difficulty too.  One
// solution was empty and one used .map and friends.
//
// Some used format! to construct the result but others use String.push().
// No one seemed to have trouble with subscripts like I did.  Some used:
//
//    let letters = Range::new(65, 91);
//
// but I believe this is a vector of number that does support subscripting.
//
// Some used an anonymous struct and the .0 syntax.
//
// There seemed to be a lack of understanding of random number generators.  A
// couple of solutions used the deprecated IndependentSample for no good reason.
//
// Three implemented uniqueness checks but that requires static data.  Two used
// lazy_static! but the third used thread_local! which appears not to be thread safe.
//
