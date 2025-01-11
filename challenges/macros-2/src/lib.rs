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
macro_rules! config_default_impl {
    ($type:ty, $value:expr) => {
        impl ConfigDefault for $type {
            fn get_default() -> Self {
                Self($value)
            }
        }
    };
}

config_default_impl!(ConnectionTimeout, 30);
config_default_impl!(MaxConnections, 100);
config_default_impl!(RetryAttempts, 3);
config_default_impl!(PostgresPort, 5432);
config_default_impl!(MySQLPort, 3306);
config_default_impl!(MongoPort, 27017);
config_default_impl!(RedisPort, 6379);
