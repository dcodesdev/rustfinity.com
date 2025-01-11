Rust macros are a powerful tool that can help reduce repetitive code and enforce patterns across multiple types. In this challenge, you'll use macros to implement a trait for defining default configuration values for different types, such as connection timeouts, maximum connections, and port numbers for various services.

The goal is to create a macro called `config_default_impl` that generates implementations of a trait called `ConfigDefault` for multiple types. The `ConfigDefault` trait is designed to provide a default value for specific configuration parameters.

The `ConfigDefault` trait looks like this:

```rust
pub trait ConfigDefault {
    fn get_default() -> Self;
}
```

For example:

- `ConnectionTimeout` should have a default value of `30`.
- `MaxConnections` should have a default value of `100`.
- Ports for various databases should return their respective default values, such as `5432` for Postgres and `6379` for Redis.

## Your Task

1. Implement the `ConfigDefault` trait with a method `get_default() -> Self` that returns a default configuration value.
2. Write a macro named `config_default_impl` that:
   - Accepts a type and a default value as arguments.
   - Generates the implementation of `ConfigDefault` for that type.
3. Use the macro to implement the `ConfigDefault` trait for the following types:
   - `ConnectionTimeout` with a value of `30`.
   - `MaxConnections` with a value of `100`.
   - `RetryAttempts` with a value of `3`.
   - `PostgresPort` with a value of `5432`.
   - `MySQLPort` with a value of `3306`.
   - `MongoPort` with a value of `27017`.
   - `RedisPort` with a value of `6379`.

### Requirements

1. The `ConfigDefault` trait and the macro `config_default_impl` should be defined in your code.
2. The macro should generate the required implementations for all specified types and values.
3. Test your implementations to ensure that `get_default()` returns the correct values for all types.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- The syntax for implementing a trait for a type is:
  ```rust
  impl TraitName for TypeName {
      fn function_name() -> ReturnType {
          // Implementation
      }
  }
  ```
- Use the macro `$type` to represent the type and `$value` to represent the corresponding default value.
- You can test your implementation by calling `<TypeName as ConfigDefault>::get_default()` and checking its output.

</details>
