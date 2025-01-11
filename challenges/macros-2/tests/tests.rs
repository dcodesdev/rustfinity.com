use macros_2::*;

#[test]
fn test_connection_timeout() {
    assert_eq!(<ConnectionTimeout as ConfigDefault>::get_default().0, 30);
}

#[test]
fn test_max_connections() {
    assert_eq!(<MaxConnections as ConfigDefault>::get_default().0, 100);
}

#[test]
fn test_retry_attempts() {
    assert_eq!(<RetryAttempts as ConfigDefault>::get_default().0, 3);
}

#[test]
fn test_database_ports() {
    assert_eq!(<PostgresPort as ConfigDefault>::get_default().0, 5432);
    assert_eq!(<MySQLPort as ConfigDefault>::get_default().0, 3306);
    assert_eq!(<MongoPort as ConfigDefault>::get_default().0, 27017);
    assert_eq!(<RedisPort as ConfigDefault>::get_default().0, 6379);
}
