pub struct Millimeters(pub u32);
pub struct Meters(pub u32);

// Implement the Add trait

// Example usage
pub fn main() {
    let length1 = Millimeters(1500);
    let length2 = Meters(3);

    let result = length1 + length2;
    assert_eq!(result.0, 4500); // Should assert: 1500mm + 3000mm = 4500mm
}
