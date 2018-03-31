//! Exercism Rust Track:  a solution for the gigasecond exercise

extern crate chrono;
use chrono::{DateTime, Utc, TimeZone};

/// Function after() takes a UTC date/time and returns it after adding 1e9 seconds to it.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let seconds = start.timestamp() + 1000000000;
    let nanoseconds = start.timestamp_subsec_nanos();

    Utc.timestamp(seconds, nanoseconds)
}

//
//  I found my first sortie into the world of online Rust documentation confusing.
//
//  Here is a better solution after work by others:
//
use chrono::{Duration};

pub fn _after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(1_000_000_000)
}
