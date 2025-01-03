We now have an overview of traits, how to define them and how they work, now it's time to put that knowledge to use and build a plugin system using traits.

A plugin in this system is any type that implements a specific trait. Each plugin will perform a specific task, and the system should manage a collection of these plugins, executing them in sequence. You’ll also address advanced issues like object safety and resolving potential conflicts between overlapping trait implementations.

## Your Task

Design and implement a plugin system using trait objects. You will:

1. Define a `Plugin` trait that includes methods for initialization and execution.
2. Create a `PluginManager` struct to manage plugins. It should:
   - Dynamically load plugins implementing the `Plugin` trait.
   - Allow adding and removing plugins.
   - Execute all registered plugins in sequence.
3. Address object safety to ensure the system works correctly with trait objects.

### Requirements

- The `Plugin` trait should include the following methods:
  - `fn name(&self) -> &str;` - Returns the name of the plugin.
  - `fn execute(&self);` - Executes the plugin's functionality.
- The `PluginManager` should:
  - Provide methods to add, remove, and list plugins.
  - Execute all registered plugins using their `execute` method.
- Use appropriate error handling for cases like duplicate plugins.

## Hints

<details>
    <summary>Click here to reveal hints</summary>

- **Object Safety**: Remember that traits used with trait objects must be object-safe. This means no methods with generic parameters or `Self` in non-where clause positions.
- **Dynamic Dispatch**: Store plugins in a `Vec<Box<dyn Plugin>>` for dynamic dispatch.
- **Plugin Uniqueness**: Use the `name` method to identify and ensure uniqueness among plugins.
- **Iterators**: Leverage iterators to execute plugins in sequence or to filter them by specific criteria.

</details>
