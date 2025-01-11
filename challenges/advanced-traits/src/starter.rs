use std::ops::Add;

pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        // Implement here
    }
}

// Example usage
pub fn main() {
    let length1 = Millimeters(1500);
    let length2 = Meters(3);

    let result = length1 + length2;
    println!("Result: {} mm", result.0); // Should print: Result: 4500 mm
}
