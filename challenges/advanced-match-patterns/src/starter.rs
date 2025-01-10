#[derive(Debug)]
pub enum Data {
    VariantA { x: i32, y: Option<i32> },
    VariantB(String, (i32, i32)),
    VariantC(Vec<Data>),
    VariantD,
}

pub fn analyze_data(data: Data) -> String {
    // TODO: Implement the function here
    unimplemented!()
}

// Example usage
pub fn main() {
    let data_a = Data::VariantA {
        x: 42,
        y: Some(100),
    };
    let data_b = Data::VariantB("hello".to_string(), (1, 2));
    let data_c = Data::VariantC(vec![data_a.clone(), Data::VariantD]);

    println!("{}", analyze_data(data_a)); // Example output: "VariantA with x: 42 and y: Some(100)"
    println!("{}", analyze_data(data_b)); // Example output: "VariantB with string hello and tuple (1, 2)"
    println!("{}", analyze_data(data_c)); // Example output: "VariantC with 2 elements"
    println!("{}", analyze_data(Data::VariantD)); // Example output: "VariantD"
}
