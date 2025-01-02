use std::cmp::PartialOrd;
use std::fmt::Display;

pub fn compare_and_display<T>(a: T, b: T) -> T
where
    T: Display + PartialOrd,
{
    println!("Comparing: {} and {}", a, b);
    if a > b {
        a
    } else {
        b
    }
}
