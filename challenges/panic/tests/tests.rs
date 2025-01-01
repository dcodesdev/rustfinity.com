use panic::*;

#[test]
fn test_get_database_url_valid() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://localhost");
}

#[test]
#[should_panic(expected = "DATABASE_URL environment variable is not set.")]
fn test_get_database_url_missing() {
    std::env::remove_var("DATABASE_URL");
    get_database_url();
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_invalid_prefix() {
    std::env::set_var("DATABASE_URL", "mysql://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_empty_value() {
    std::env::set_var("DATABASE_URL", "postgresql://");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://");
}

#[test]
fn test_get_database_url_with_long_valid_url() {
    let long_url = "postgresql://user:password@host:5432/database";
    std::env::set_var("DATABASE_URL", long_url);
    let db_url = get_database_url();
    assert_eq!(db_url, long_url);
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_prefix_case_sensitive() {
    std::env::set_var("DATABASE_URL", "PostgreSQL://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_with_special_characters() {
    let special_url = "postgresql://user:pa$$word@host:5432/dbname";
    std::env::set_var("DATABASE_URL", special_url);
    let db_url = get_database_url();
    assert_eq!(db_url, special_url);
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_numeric_prefix() {
    std::env::set_var("DATABASE_URL", "123postgresql://localhost");
    get_database_url();
}

#[test]
fn test_get_database_url_edge_case_minimal_valid() {
    std::env::set_var("DATABASE_URL", "postgresql://h");
    let db_url = get_database_url();
    assert_eq!(db_url, "postgresql://h");
}

#[test]
#[should_panic(expected = "DATABASE_URL must start with 'postgresql://'")]
fn test_get_database_url_unexpected_trailing_characters() {
    std::env::set_var("DATABASE_URL", "postgresql://localhost!invalid");
    get_database_url();
}

#[test]
fn test_get_database_url_with_port_only() {
    let port_only_url = "postgresql://:5432";
    std::env::set_var("DATABASE_URL", port_only_url);
    let db_url = get_database_url();
    assert_eq!(db_url, port_only_url);
}
