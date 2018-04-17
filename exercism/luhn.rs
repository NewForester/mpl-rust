//! Exercism Rust Track:  a solution for the luhn exercise

/// Function luhn::is_valid() takes a SIN and returns true if it is valid
pub fn is_valid(number: &str) -> bool {
    let mut luhnsum = 0;
    let mut digitcount = 0;

    for cc in number.chars() {
        if cc.is_digit(10) {
            digitcount += 1;

            let mut luhndigit = cc.to_digit(10).unwrap();

            if digitcount % 2 == 0 {
                luhndigit *= 2;
                if luhndigit > 9 {
                    luhndigit -= 9;
                }
            }

            luhnsum += luhndigit;
        }
        else if cc != ' ' {
            return false;
        }
    }

    digitcount > 1 && luhnsum % 10 == 0
}

//
// You could say this solution is too like Go or even C.
//
// All the other solutions used map and friends, which seems overkill.
//
