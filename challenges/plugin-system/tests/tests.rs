use plugin_system::*;

struct TestPlugin {
    name: String,
}

impl TestPlugin {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Plugin for TestPlugin {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute(&self) {
        println!("Executing plugin: {}", self.name);
    }
}

#[test]
fn test_add_and_execute_plugins() {
    let mut manager = PluginManager::new();

    let plugin1 = Box::new(TestPlugin::new("Plugin1"));
    let plugin2 = Box::new(TestPlugin::new("Plugin2"));

    manager.add_plugin(plugin1);
    manager.add_plugin(plugin2);

    manager.execute_all();
}

#[test]
#[should_panic(expected = "Plugin with name 'Plugin1' already exists")]
fn test_add_duplicate_plugin() {
    let mut manager = PluginManager::new();

    let plugin1 = Box::new(TestPlugin::new("Plugin1"));
    let duplicate_plugin = Box::new(TestPlugin::new("Plugin1"));

    manager.add_plugin(plugin1);
    manager.add_plugin(duplicate_plugin); // Should panic
}

#[test]
fn test_remove_plugin() {
    let mut manager = PluginManager::new();

    let plugin1 = Box::new(TestPlugin::new("Plugin1"));
    manager.add_plugin(plugin1);

    let removed = manager.remove_plugin("Plugin1");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().name(), "Plugin1");

    assert!(manager.remove_plugin("Plugin1").is_none()); // Already removed
}

#[test]
fn test_execute_with_no_plugins() {
    let manager = PluginManager::new();
    manager.execute_all(); // Should not panic and should do nothing
}

#[test]
fn test_remove_non_existent_plugin() {
    let mut manager = PluginManager::new();
    assert!(manager.remove_plugin("NonExistent").is_none());
}

#[test]
fn test_add_multiple_plugins() {
    let mut manager = PluginManager::new();

    for i in 1..=5 {
        let plugin = Box::new(TestPlugin::new(&format!("Plugin{}", i)));
        manager.add_plugin(plugin);
    }

    assert_eq!(manager.plugins.len(), 5);
}
