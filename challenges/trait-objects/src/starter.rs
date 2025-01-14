pub trait Renderable {
    fn render(&self) -> String;
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

// 1. Implement the trait for Circle and Rectangle

// 2. Create the Canvas struct

// 3. Implement the Canvas struct

// Example usage
pub fn main() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    for shape in rendered_shapes {
        println!("{}", shape);
    }
}
