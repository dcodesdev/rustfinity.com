pub trait DefaultValue {
    fn default_value() -> Self;
}

macro_rules! default_value_impl {
    ($type:ty, $value:expr) => {
        impl DefaultValue for $type {
            fn default_value() -> Self {
                $value
            }
        }
    };
}

// Use the macro to implement DefaultValue for multiple types
default_value_impl!(f64, 0.0);
default_value_impl!(f32, 0.0);
default_value_impl!(u32, 2147483647);
default_value_impl!(u8, 127);
default_value_impl!(i32, 0);
default_value_impl!(u16, 32767);
default_value_impl!(i16, 0);
default_value_impl!(i8, 0);
