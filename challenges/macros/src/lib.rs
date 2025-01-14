#[macro_export]
macro_rules! math_operations {
    ($a:expr, $op:expr, $b:expr) => {{
        let result = match $op {
            "+" => $a + $b,
            "-" => $a - $b,
            "*" => $a * $b,
            "/" => {
                if $b == 0 {
                    panic!("Division by zero");
                }
                $a / $b
            }
            _ => panic!("Unsupported operator: {}", $op),
        };
        format!("{} {} {} = {}", $a, $op, $b, result)
    }};
}

// Example usage
pub fn main() {
    assert_eq!(math_operations!(4, "+", 2), "4 + 2 = 6");
    assert_eq!(math_operations!(10, "-", 3), "10 - 3 = 7");
    assert_eq!(math_operations!(6, "*", 4), "6 * 4 = 24");
    assert_eq!(math_operations!(15, "/", 3), "15 / 3 = 5");
}
