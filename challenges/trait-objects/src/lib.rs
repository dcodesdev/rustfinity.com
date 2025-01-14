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
        format!("Circle with radius {}", self.radius)
    }
}

impl Renderable for Rectangle {
    fn render(&self) -> String {
        format!(
            "Rectangle with width {} and height {}",
            self.width, self.height
        )
    }
}

pub struct Canvas {
    shapes: Vec<Box<dyn Renderable>>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Renderable>) {
        self.shapes.push(shape);
    }

    pub fn render_all(&self) -> Vec<String> {
        self.shapes.iter().map(|shape| shape.render()).collect()
    }
}
