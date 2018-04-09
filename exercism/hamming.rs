//! Exercism Rust Track:  a solution for the hamming exercise

/// Function hamming_distance returns the hamming distance between to strands of DNA
pub fn hamming_distance(a: &str, b: &str) -> Result<u32,&'static str> {
    let ab = a.as_bytes();
    let bb = b.as_bytes();

    if ab.len() != bb.len() {
        return Err("Strands of different lengths");
    }

    let mut count = 0;
    for ii in 0..bb.len() {
        count += if ab[ii] != bb[ii] {1} else {0};
    }

    Ok(count)
}

//
// All 8 other submissions examined used function programming constructs.
// Some used filter, others fold but all used chars and zip.
// Here is an example:
//
pub fn _hamming_distance(a: &str, b: &str) -> Result<usize, ()> {
    if a.len() != b.len() {
        return Err(());
    }
    Ok(a.chars().zip(b.chars()).filter(|&(x, y)| x != y).count())
}
//
// I used as_bytes where as everyone else used chars.  I guess their code is
// the clearer.  Their solutions are not valid for Unicode strings.
//
