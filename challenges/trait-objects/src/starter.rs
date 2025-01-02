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

impl Renderable for Circle {
    fn render(&self) -> String {
        // Implement here
        unimplemented!()
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> String {
        // Implement here
        unimplemented!()
    }
}

pub struct Canvas {
    shapes: Vec<Box<dyn Renderable>>,
}

impl Canvas {
    pub fn new() -> Self {
        // Implement here
        unimplemented!()
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        // Implement here
        unimplemented!()
    }

    pub fn render_all(&self) -> Vec<String> {
        // Implement here
        unimplemented!()
    }
}

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
