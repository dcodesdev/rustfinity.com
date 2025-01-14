pub trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

pub struct PluginManager {
    pub plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn add_plugin(&mut self, plugin: Box<dyn Plugin>) {
        if self.plugins.iter().any(|p| p.name() == plugin.name()) {
            panic!("Plugin with name '{}' already exists", plugin.name());
        }
        self.plugins.push(plugin);
    }

    pub fn remove_plugin(&mut self, name: &str) -> Option<Box<dyn Plugin>> {
        if let Some(pos) = self.plugins.iter().position(|p| p.name() == name) {
            Some(self.plugins.remove(pos))
        } else {
            None
        }
    }

    pub fn execute_all(&self) {
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}
