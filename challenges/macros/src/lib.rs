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
