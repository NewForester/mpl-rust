//! Exercism Rust Track:  a solution for the space-age exercise

const EARTH_YEAR: f64 = 31_557_600 as f64;

pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;         // in earth years

    fn years_during(d: &Duration) -> f64 {
       d.0 / Self::ORBITAL_PERIOD / EARTH_YEAR
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD: f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_PERIOD: f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_PERIOD: f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD: f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD: f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD: f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD: f64 = 164.79132;
}

//
// The instructions invited us to change whatever we liked but the interface
// expected by the test cases meant I could not rewrite this as I wished.
//
// I believe I was forced into implementing a trait, which, in this case,
// appears overkill.  It certainly appears verbose.
//
// All 8 other solutions examined also implemented a trait. This is my
// second attempt and incorporates minor changes from several other solutions.
// It is about a dozen lines shorter than my first.
//
// On the upside, the amount of experimentation I was forced to use to arrive
// at this solution means I learnt a lot more about Rust syntax than one
// might expect for such a simple exercise.
//
// Here's hoping the exercises that follow involve the same effort.
//
