use macros_2::*;

#[test]
fn test_connection_timeout() {
    assert_eq!(
        <ConnectionTimeout as ConfigDefault>::get_default().0,
        30,
        "Connection timeout should default to 30 seconds"
    );
}

#[test]
fn test_max_connections() {
    assert_eq!(
        <MaxConnections as ConfigDefault>::get_default().0,
        100,
        "Max connections should default to 100"
    );
}

#[test]
fn test_retry_attempts() {
    assert_eq!(
        <RetryAttempts as ConfigDefault>::get_default().0,
        3,
        "Retry attempts should default to 3"
    );
}

#[test]
fn test_database_ports() {
    assert_eq!(
        <PostgresPort as ConfigDefault>::get_default().0,
        5432,
        "Postgres should use default port 5432"
    );
    assert_eq!(
        <MySQLPort as ConfigDefault>::get_default().0,
        3306,
        "MySQL should use default port 3306"
    );
    assert_eq!(
        <MongoPort as ConfigDefault>::get_default().0,
        27017,
        "MongoDB should use default port 27017"
    );
    assert_eq!(
        <RedisPort as ConfigDefault>::get_default().0,
        6379,
        "Redis should use default port 6379"
    );
}

#[test]
fn test_timeout_conversion() {
    let timeout = <ConnectionTimeout as ConfigDefault>::get_default();
    assert_eq!(
        timeout.0 * 1000,
        30000,
        "30 seconds should convert to 30000 milliseconds"
    );
}

#[test]
fn test_ports_are_valid_range() {
    let ports = [
        <PostgresPort as ConfigDefault>::get_default().0,
        <MySQLPort as ConfigDefault>::get_default().0,
        <MongoPort as ConfigDefault>::get_default().0,
        <RedisPort as ConfigDefault>::get_default().0,
    ];

    for port in ports {
        assert!(port > 0, "Port number must be positive");
        assert!(port < 65535, "Port number exceeds valid range");
    }
}

#[test]
fn test_max_connections_reasonable() {
    let max_conn = <MaxConnections as ConfigDefault>::get_default().0;
    assert!(max_conn >= 50, "Max connections should be at least 50");
    assert!(max_conn <= 1000, "Max connections shouldn't exceed 1000");
}

#[test]
fn test_retry_attempts_bounds() {
    let retries = <RetryAttempts as ConfigDefault>::get_default().0;
    assert!(retries > 0, "Should have at least 1 retry");
    assert!(retries <= 10, "Shouldn't have excessive retries");
}

// Test configuration composition
struct DatabaseConfig {
    timeout: ConnectionTimeout,
    max_conn: MaxConnections,
    retries: RetryAttempts,
    port: PostgresPort,
}

impl DatabaseConfig {
    fn new_with_defaults() -> Self {
        Self {
            timeout: ConfigDefault::get_default(),
            max_conn: ConfigDefault::get_default(),
            retries: ConfigDefault::get_default(),
            port: ConfigDefault::get_default(),
        }
    }
}

#[test]
fn test_config_composition() {
    let config = DatabaseConfig::new_with_defaults();
    assert_eq!(
        config.timeout.0, 30,
        "Composed config should have default timeout"
    );
    assert_eq!(
        config.max_conn.0, 100,
        "Composed config should have default max connections"
    );
    assert_eq!(
        config.retries.0, 3,
        "Composed config should have default retry attempts"
    );
    assert_eq!(
        config.port.0, 5432,
        "Composed config should have default Postgres port"
    );
}

#[test]
fn test_all_ports_different() {
    let ports = [
        <PostgresPort as ConfigDefault>::get_default().0,
        <MySQLPort as ConfigDefault>::get_default().0,
        <MongoPort as ConfigDefault>::get_default().0,
        <RedisPort as ConfigDefault>::get_default().0,
    ];

    // Check that all ports are unique
    for i in 0..ports.len() {
        for j in i + 1..ports.len() {
            assert_ne!(
                ports[i], ports[j],
                "Port values must be unique - found duplicate: {}",
                ports[i]
            );
        }
    }
}

// Use the macro to do more impl
struct CustomPort(pub u16);

config_default_impl!(CustomPort, 8080);

#[test]
fn test_custom_port() {
    assert_eq!(
        <CustomPort as ConfigDefault>::get_default().0,
        8080,
        "Custom port should default to 8080"
    );
}
