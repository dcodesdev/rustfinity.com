pub fn get_database_url() -> String {
    let result = std::env::var("DATABASE_URL");

    match result {
        Ok(value) => {
            if value.starts_with("postgresql://") {
                value
            } else {
                panic!("DATABASE_URL must start with 'postgresql://'");
            }
        }
        Err(_) => panic!("DATABASE_URL environment variable is not set."),
    }
}
