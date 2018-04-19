//! Exercism Rust Track:  a solution for the pythagorean-triplet exercise

/// Function find() returns the product of the Pythagorean triplet whose sum is 1000
pub fn find() -> Option<u32> {
    /*
       From Euclid's formula for Pythagorean triplet was have:

           aa = 2 * mm * nn

           bb = mm * mm - nn * nn

           cc = mm * mm + nn * nn

       where the constraint is:

           aa + bb + cc = 1000

       substituting for aa, bb and cc we have:

           1000 = (2 * mm * nn) + (mm * mm - nn * nn) + (mm * mm + nn * nn)

       that can be reduced to:

           nn = ((1000 / 2) / mm) - mm

       the rest is brute force (and very quick)
    */
    for mm in 1_i32 .. {
        let nn: i32 = (500 / mm) - mm;

        if nn < 0 {
            break
        }

        let (aa, bb, cc) = euclid(nn as u32, mm as u32);

        if aa+bb+cc == 1000 {
            return Some(aa * bb * cc)
        }
    }

    return None // does not happen
}

/// Function euclid() takes two numbes (in principle coprime) and return a Pythagorean triplet
fn euclid(nn: u32, mm: u32) -> (u32, u32, u32) {
    let (xx, yy) = if mm < nn {
        (nn, mm)
    }
    else {
        (mm, nn)
    };

    let aa = 2 * xx * yy;

    let bb = xx * xx - yy * yy;

    let cc = xx * xx + yy * yy;

    if bb < aa {
        (bb, aa, cc)
    }
    else {
        (aa, bb, cc)
    }
}

//
// Well, at least this track only wanted an answer to the problem.  That meant
// only one test that has the answer in plain text so one person just returned
// that.  O marks for that.
//
// What was interesting was that only one person used map and friends.  Even
// more interesting is that you can use map and friends.  Perhaps some other
// day.
//
// The others used a brute force approach.  These solutions look short and
// neat but then they are not solving a general problem.
//
// The http://exercism.io/submissions/fbd7c62e132f4867940797dc82f29602 solution
// used a one pass loop, which makes it unique and worth further study.  The
// algorithm probably also allows for the use of map and friends.
//
