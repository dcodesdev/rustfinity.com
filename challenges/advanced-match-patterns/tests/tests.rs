use advanced_match_patterns::*;

#[test]
fn test_variant_a_some() {
    let data = Data::VariantA {
        x: 42,
        y: Some(100),
    };
    assert_eq!(analyze_data(data), "VariantA with x: 42 and y: 100");
}

#[test]
fn test_variant_a_none() {
    let data = Data::VariantA { x: 42, y: None };
    assert_eq!(analyze_data(data), "VariantA with x: 42 and y: None");
}

#[test]
fn test_variant_b() {
    let data = Data::VariantB("hello".to_string(), (1, 2));
    assert_eq!(
        analyze_data(data),
        "VariantB with string 'hello' and tuple (1, 2)"
    );
}

#[test]
fn test_variant_c() {
    let data = Data::VariantC(vec![Data::VariantA { x: 10, y: None }, Data::VariantD]);
    assert_eq!(analyze_data(data), "VariantC with 2 elements");
}

#[test]
fn test_variant_d() {
    let data = Data::VariantD;
    assert_eq!(analyze_data(data), "VariantD");
}

#[test]
fn test_variant_b_special_chars() {
    let data = Data::VariantB("Hello, 世界!".to_string(), (-1, 99));
    assert_eq!(
        analyze_data(data),
        "VariantB with string 'Hello, 世界!' and tuple (-1, 99)"
    );
}

#[test]
fn test_variant_c_deeply_nested() {
    let data = Data::VariantC(vec![
        Data::VariantC(vec![Data::VariantA { x: 1, y: Some(2) }, Data::VariantD]),
        Data::VariantB("nested".to_string(), (5, 6)),
    ]);
    assert_eq!(analyze_data(data), "VariantC with 2 elements");
}
