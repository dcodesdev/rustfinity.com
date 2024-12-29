pub struct Rectangle(pub u32, pub u32);

pub fn area(rect: &Rectangle) -> u32 {
    rect.0 * rect.1
}
