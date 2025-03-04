use std::ops::Not;
pub use chrono::{Datelike, NaiveDate, Weekday as wd};

// If the year is evenly divisible by 4, go to step 2. Otherwise, go to step 5.
// If the year is evenly divisible by 100, go to step 3. Otherwise, go to step 4.
// If the year is evenly divisible by 400, go to step 4. Otherwise, go to step 5.
// The year is a leap year (it has 366 days).
// The year is not a leap year (it has 365 days).


pub fn is_leap_year(year: u64) -> bool {
    if year % 4 == 0 {
        if year % 100 == 0 {
            if year % 400 == 0 {
                true
            } else {
                false
            }
        } else {
            true
        }
    } else {
        false
    }
}

const MIDDLE_DAY: u32 = 183;
pub fn middle_day(year: u64) -> Option<wd> {
    is_leap_year(year).not()
        .then(|| NaiveDate::from_yo_opt(year as i32, MIDDLE_DAY))
        .flatten()
        .map(|d| d.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Not;
    use chrono::Weekday;

    #[test]
    fn leap_year_test() {
        assert!(is_leap_year(1022).not());
        assert!(is_leap_year(1992));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1900).not());
    }

    #[test]
    fn it_works() {
        assert_eq!(Some(Weekday::Thu), middle_day(1022));
    }
}

// Instructions
//
// Use the chrono crate to create a function named middle_day. It accepts a year, and returns the weekday of the middle day of that year, wrapped in an Option. chrono::Weekday has to be referred to as wd.
//
// Years with an even number of days do not have a middle day, and should return None.
// Expected Function
//
// You'll need to work out the function signature for yourself.
//
// Usage
//
// Here is a program to test your function:
//
// use middle_day::*;
//
// fn main() {
//     println!("{:?}", middle_day(1022).unwrap());
// }
//
// And its output:
//
// $ cargo run
// Tue
// $
//
