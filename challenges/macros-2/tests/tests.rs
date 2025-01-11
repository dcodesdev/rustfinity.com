use macros_2::*;

#[test]
fn test_default_value_f64() {
    assert_eq!(<f64 as DefaultValue>::default_value(), 0.0);
}

#[test]
fn test_default_value_u32() {
    assert_eq!(<u32 as DefaultValue>::default_value(), 2147483647);
}

#[test]
fn test_default_value_u8() {
    assert_eq!(<u8 as DefaultValue>::default_value(), 127);
}

#[test]
fn test_default_value_i32() {
    assert_eq!(<i32 as DefaultValue>::default_value(), 0);
}

#[test]
fn test_default_value_u16() {
    assert_eq!(<u16 as DefaultValue>::default_value(), 32767);
}

#[test]
fn test_default_value_i16() {
    assert_eq!(<i16 as DefaultValue>::default_value(), 0);
}

#[test]
fn test_default_value_i8() {
    assert_eq!(<i8 as DefaultValue>::default_value(), 0);
}
