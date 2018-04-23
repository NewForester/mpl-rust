//! Exercism Rust Track:  a solution for the parallel letter frequency exercise

use std::thread;
use std::sync::mpsc;
use std::collections::HashMap;

type FreqMap = HashMap<char, u64>;

/// Function frequency() takes a load of strings and returns a hash map containing letter frequencies: uses concurrency
pub fn frequency(strings: &[&'static str],  nn: usize) -> FreqMap {
    let (tx, rx) = mpsc::channel();

    for ii in 0 .. nn {
        let lines = sorter(strings, ii, nn);

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            tx1.send(teller(&lines)).unwrap();
        });
    }

    let mut map = HashMap::new();

    for _ in 0 .. nn {
        for (kk, vv) in rx.recv().unwrap() {
            *map.entry(kk).or_insert(0) += vv;
        }
    }

    map
}

/// Function sorter() takes a load of strings and returns a stripe across those strings
fn sorter(strings: &[&'static str], mut ii :usize, nn: usize) -> Vec<&'static str> {
    let mut lines = Vec::new();

    while ii < strings.len() {
        lines.push(strings[ii]);
        ii += nn;
    }

    lines
}

/// Function teller() takes a load of strings and returns a hash map containing letter frequencies
fn teller(strings: &[&str]) -> FreqMap {
    let mut map = HashMap::new();

    for ss in strings {
        for cc in ss.to_lowercase().chars() {
            if cc.is_alphabetic() {
                *map.entry(cc).or_insert(0) += 1;
            }
        }
    }

    map
}

//
// My first piece of Rust code to use concurrent execution.  This may be a toy
// that does not actually execute the test cases any faster than a sequential
// solution does.
//
// This was written after submitting a sequential solution and having a peek
// at other solutions.  This partly to ensure I had understood the problem
// since it is not the same as the Go equivalent.
//
// The solutions examined all followed the same sort of pattern but each looked
// very different.  Some used the same crates as I did; others knew better.
//
// My solution is neither significantly longer or shorter than the others but,
// somehow, to me, it appears less dense and simpler.  I suspect that may be
// simply because the other solutions have not been refactored.
//
// I went as far as I could with a sequential solution by refactoring it
// several times before trying to convert to a concurrent model.
//
// Why is everything so hard in Rust ?  It did not think my code was safe.
// Was it that it could see something I couldn't or the other way around ?
//
// It seemed most upset about lifetimes.  It could not see that my threads
// would all exit before my routine returned.  An omission on my part ? Dunno.
// The solutions examined were of no help - far too much confusion there.
//
// I could not simply pass the strings parameter down to the threads:  I needed
// to rearrange the references, which I did by moving some 'read only' code
// out of the thread.  Other solutions appeared have rearranged the input as
// well but whether this was deliberate or serendipitous I could not tell.
//
// My solution used stripes - each thread dealt with every nth string.  Most
// of the others deal with chunks - groups of n strings.  I have no idea how
// much real data is copied but most of the chunk create code looked overly
// complex.
//
// The final result is 60 lines; the sequential solution from which it derives
// was only 20.
//
