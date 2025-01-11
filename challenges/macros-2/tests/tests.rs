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

#[test]
fn test_timeout_conversion() {
    let timeout = <ConnectionTimeout as ConfigDefault>::get_default();
    // 30 seconds to milliseconds
    assert_eq!(timeout.0 * 1000, 30000);
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
        assert!(port > 0);
        assert!(port < 65535);
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
    assert_eq!(config.timeout.0, 30);
    assert_eq!(config.max_conn.0, 100);
    assert_eq!(config.retries.0, 3);
    assert_eq!(config.port.0, 5432);
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
            assert_ne!(ports[i], ports[j], "Ports should be unique");
        }
    }
}
