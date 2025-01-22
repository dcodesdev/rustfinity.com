use trait_objects::*;

#[test]
fn test_circle_render() {
    let circle = Circle { radius: 5.0 };
    assert_eq!(circle.render(), "Circle with radius 5");
}

#[test]
fn test_rectangle_render() {
    let rectangle = Rectangle {
        width: 3.0,
        height: 4.0,
    };
    assert_eq!(rectangle.render(), "Rectangle with width 3 and height 4");
}

#[test]
fn test_canvas_add_and_render() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 5.0 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 3.0,
        height: 4.0,
    }));
    let rendered_shapes = canvas.render_all();
    assert_eq!(
        rendered_shapes,
        vec![
            "Circle with radius 5",
            "Rectangle with width 3 and height 4"
        ]
    );
}

#[test]
fn test_empty_canvas_render() {
    let canvas = Canvas::new();
    let rendered_shapes = canvas.render_all();
    assert!(rendered_shapes.is_empty(), "Canvas should start empty");
}

#[test]
fn test_add_multiple_shapes() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 2.5 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 4.0,
        height: 6.0,
    }));
    canvas.add_shape(Box::new(Circle { radius: 7.0 }));

    let rendered_shapes = canvas.render_all();
    assert_eq!(
        rendered_shapes,
        vec![
            "Circle with radius 2.5",
            "Rectangle with width 4 and height 6",
            "Circle with radius 7"
        ]
    );
}

#[test]
fn test_large_values() {
    let circle = Circle { radius: 1e9 };
    let rectangle = Rectangle {
        width: 1e9,
        height: 2e9,
    };

    assert_eq!(circle.render(), "Circle with radius 1000000000");
    assert_eq!(
        rectangle.render(),
        "Rectangle with width 1000000000 and height 2000000000"
    );
}

#[test]
fn test_zero_values() {
    let circle = Circle { radius: 0.0 };
    let rectangle = Rectangle {
        width: 0.0,
        height: 0.0,
    };

    assert_eq!(circle.render(), "Circle with radius 0");
    assert_eq!(rectangle.render(), "Rectangle with width 0 and height 0");
}

#[test]
fn test_negative_values() {
    let circle = Circle { radius: -3.0 };
    let rectangle = Rectangle {
        width: -4.0,
        height: -5.0,
    };

    assert_eq!(circle.render(), "Circle with radius -3");
    assert_eq!(rectangle.render(), "Rectangle with width -4 and height -5");
}

#[test]
fn test_canvas_with_large_and_zero_shapes() {
    let mut canvas = Canvas::new();
    canvas.add_shape(Box::new(Circle { radius: 1e9 }));
    canvas.add_shape(Box::new(Rectangle {
        width: 0.0,
        height: 0.0,
    }));

    let rendered_shapes = canvas.render_all();
    assert_eq!(
        rendered_shapes,
        vec![
            "Circle with radius 1000000000",
            "Rectangle with width 0 and height 0"
        ]
    );
}

#[test]
fn test_render_lots_of_items() {
    let mut canvas = Canvas::new();
    for i in 0..1000 {
        if i % 2 == 0 {
            canvas.add_shape(Box::new(Circle { radius: i as f64 }));
        } else {
            canvas.add_shape(Box::new(Rectangle {
                width: i as f64,
                height: i as f64,
            }));
        }
    }

    let rendered_shapes = canvas.render_all();
    for i in 0..1000 {
        if i % 2 == 0 {
            assert_eq!(rendered_shapes[i], format!("Circle with radius {}", i));
        } else {
            assert_eq!(
                rendered_shapes[i],
                format!("Rectangle with width {} and height {}", i, i)
            );
        }
    }
}
