use trait_bounds::*;

#[test]
fn test_compare_integers() {
    assert_eq!(compare_and_display(10, 20), 20);
    assert_eq!(compare_and_display(30, 15), 30);
    assert_eq!(compare_and_display(-5, -10), -5);
    assert_eq!(compare_and_display(0, 0), 0);
}

#[test]
fn test_compare_floats() {
    assert_eq!(compare_and_display(3.14, 2.71), 3.14);
    assert_eq!(compare_and_display(-1.0, -2.0), -1.0);
    assert_eq!(compare_and_display(0.0, 0.0), 0.0);
}

#[test]
fn test_compare_strings() {
    assert_eq!(compare_and_display("Apple", "Orange"), "Orange");
    assert_eq!(compare_and_display("Zebra", "Yak"), "Zebra");
    assert_eq!(compare_and_display("abc", "xyz"), "xyz");
    assert_eq!(compare_and_display("", "NonEmpty"), "NonEmpty");
}

#[test]
fn test_compare_characters() {
    assert_eq!(compare_and_display('a', 'b'), 'b');
    assert_eq!(compare_and_display('z', 'y'), 'z');
    assert_eq!(compare_and_display('!', '@'), '@');
}

#[test]
fn test_compare_custom_struct() {
    #[derive(Debug, PartialOrd, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl std::fmt::Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Point({}, {})", self.x, self.y)
        }
    }

    let point1 = Point { x: 1, y: 2 };
    let point2 = Point { x: 3, y: 4 };

    assert_eq!(compare_and_display(point1, point2), Point { x: 3, y: 4 });
}

#[test]
fn test_compare_equal_custom_struct() {
    #[derive(Debug, PartialOrd, PartialEq)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl std::fmt::Display for Rectangle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Rectangle({}, {})", self.width, self.height)
        }
    }

    let rect1 = Rectangle {
        width: 4,
        height: 5,
    };
    let rect2 = Rectangle {
        width: 4,
        height: 5,
    };

    assert_eq!(
        compare_and_display(rect1, rect2),
        Rectangle {
            width: 4,
            height: 5
        }
    );
}

#[test]
fn test_compare_different_magnitude_custom_struct() {
    #[derive(Debug, PartialOrd, PartialEq)]
    struct Circle {
        radius: f64,
    }

    impl std::fmt::Display for Circle {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Circle({})", self.radius)
        }
    }

    let circle1 = Circle { radius: 1.5 };
    let circle2 = Circle { radius: 2.5 };

    assert_eq!(
        compare_and_display(circle1, circle2),
        Circle { radius: 2.5 }
    );
}
