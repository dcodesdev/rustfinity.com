pub trait Plugin {
    // 1. Finish the trait
}

pub struct PluginManager {
    // 2. Finish the struct
    // Make fields public
}

// 3. Implement the PluginManager
impl PluginManager {}

// Example usage
pub fn main() {
    let mut manager = PluginManager::new();

    manager.add_plugin(Box::new(MyPlugin::new()));
    manager.execute_all();
}
