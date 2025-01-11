pub trait ConfigDefault {
    fn get_default() -> Self;
}

pub struct ConnectionTimeout(pub u64);
pub struct MaxConnections(pub u32);
pub struct RetryAttempts(pub u8);
pub struct PostgresPort(pub u16);
pub struct MySQLPort(pub u16);
pub struct MongoPort(pub u16);
pub struct RedisPort(pub u16);

#[macro_export]
macro_rules! default_value_impl {
    // Your code here
}

// Example usage
pub fn main() {
    // let's say we have a new struct
    struct CustomPort(pub u16);

    // we implement the ConfigDefault trait for CustomPort
    config_default_impl!(CustomPort, 8080);

    // when running the `get_default` method, it should return the default value
    assert_eq!(<CustomPort as ConfigDefault>::get_default().0, 8080);
}
