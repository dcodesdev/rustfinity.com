#[derive(Debug)]
pub enum Data {
    VariantA { x: i32, y: Option<i32> },
    VariantB(String, (i32, i32)),
    VariantC(Vec<Data>),
    VariantD,
}

pub fn analyze_data(data: Data) -> String {
    match data {
        Data::VariantA { x, y: Some(y) } => format!("VariantA with x: {} and y: {}", x, y),
        Data::VariantA { x, y: None } => format!("VariantA with x: {} and y: None", x),
        Data::VariantB(s, (a, b)) => {
            format!("VariantB with string '{}' and tuple ({}, {})", s, a, b)
        }
        Data::VariantC(contents) => format!("VariantC with {} elements", contents.len()),
        Data::VariantD => "VariantD".to_string(),
    }
}
