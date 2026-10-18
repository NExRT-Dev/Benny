use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;
use std::collections::HashSet;

pub struct Engine {
    plugins: Vec<Box<dyn Plugin>>,
    context: Context,
    enabled: HashSet<String>,
    initialized: bool,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
            context: Context::new(),
            enabled: HashSet::new(),
            initialized: false,
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn register<P: Plugin + 'static>(&mut self, plugin: P) {
        let name = plugin.name().to_string();
        self.plugins.push(Box::new(plugin));
        self.enabled.insert(name);
    }

    pub fn is_registered(&self, name: &str) -> bool {
        self.plugins.iter().any(|p| p.name() == name)
    }

    pub fn is_enabled(&self, name: &str) -> bool {
        self.enabled.contains(name)
    }

    pub fn enable(&mut self, name: &str) -> Result<(), EngineError> {
        if !self.is_registered(name) {
            return Err(EngineError::NotFound(name.to_string()));
        }
        self.enabled.insert(name.to_string());
        Ok(())
    }

    pub fn disable(&mut self, name: &str) {
        self.enabled.remove(name);
    }

    pub fn plugin_names(&self) -> Vec<&str> {
        self.plugins.iter().map(|p| p.name()).collect()
    }

    pub fn plugins_info(&self) -> Vec<(&str, &[&str])> {
        self.plugins.iter().map(|p| (p.name(), p.tags())).collect()
    }

    pub fn init_all(&mut self) -> Result<(), EngineError> {
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if let Err(e) = self.plugins[i].init(&mut self.context) {
                return Err(EngineError::PluginInit {
                    name,
                    message: e.to_string(),
                });
            }
        }
        self.initialized = true;
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), EngineError> {
        if !self.initialized {
            return Err(EngineError::NotInitialized);
        }
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if let Err(e) = self.plugins[i].run(&mut self.context) {
                return Err(EngineError::PluginRun {
                    name,
                    message: e.to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn shutdown(&mut self) {
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if let Err(e) = self.plugins[i].shutdown(&mut self.context) {
                eprintln!("plugin '{name}' shutdown error: {e}");
            }
        }
        self.initialized = false;
    }

    /// Initialize all enabled plugins, logging and disabling any that fail.
    pub fn init_best_effort(&mut self) -> usize {
        let mut failures = 0;
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if let Err(e) = self.plugins[i].init(&mut self.context) {
                eprintln!("plugin '{name}' init failed: {e}");
                self.enabled.remove(&name);
                failures += 1;
            }
        }
        self.initialized = true;
        failures
    }

    /// Run all enabled plugins, logging failures instead of aborting.
    pub fn run_best_effort(&mut self) -> usize {
        if !self.initialized {
            eprintln!("engine not initialized");
            return self.plugins.len();
        }
        let mut failures = 0;
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if let Err(e) = self.plugins[i].run(&mut self.context) {
                eprintln!("plugin '{name}' run failed: {e}");
                failures += 1;
            }
        }
        failures
    }

    /// Returns names of plugins whose tags contain `tag`.
    pub fn plugins_with_tag(&self, tag: &str) -> Vec<&str> {
        self.plugins
            .iter()
            .filter(|p| p.tags().contains(&tag))
            .map(|p| p.name())
            .collect()
    }

    pub fn init_tag(&mut self, tag: &str) -> Result<(), EngineError> {
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if !self.plugins[i].tags().contains(&tag) {
                continue;
            }
            if let Err(e) = self.plugins[i].init(&mut self.context) {
                return Err(EngineError::PluginInit {
                    name,
                    message: e.to_string(),
                });
            }
        }
        self.initialized = true;
        Ok(())
    }

    pub fn run_tag(&mut self, tag: &str) -> Result<(), EngineError> {
        if !self.initialized {
            return Err(EngineError::NotInitialized);
        }
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if !self.plugins[i].tags().contains(&tag) {
                continue;
            }
            if let Err(e) = self.plugins[i].run(&mut self.context) {
                return Err(EngineError::PluginRun {
                    name,
                    message: e.to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn init_names(&mut self, names: &[String]) -> usize {
        let mut failures = 0;
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if !names.iter().any(|n| n == &name) {
                continue;
            }
            if let Err(e) = self.plugins[i].init(&mut self.context) {
                eprintln!("plugin '{name}' init failed: {e}");
                self.enabled.remove(&name);
                failures += 1;
            }
        }
        self.initialized = true;
        failures
    }

    pub fn run_names(&mut self, names: &[String]) -> usize {
        if !self.initialized {
            eprintln!("engine not initialized");
            return names.len();
        }
        let mut failures = 0;
        for i in 0..self.plugins.len() {
            let name = self.plugins[i].name().to_string();
            if !self.enabled.contains(&name) {
                continue;
            }
            if !names.iter().any(|n| n == &name) {
                continue;
            }
            if let Err(e) = self.plugins[i].run(&mut self.context) {
                eprintln!("plugin '{name}' run failed: {e}");
                failures += 1;
            }
        }
        failures
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct CounterPlugin {
        name: &'static str,
        runs: Arc<AtomicUsize>,
    }

    impl Plugin for CounterPlugin {
        fn name(&self) -> &str {
            self.name
        }
        fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
            self.runs.fetch_add(1, Ordering::SeqCst);
            let n = self.runs.load(Ordering::SeqCst);
            ctx.set("runs", n.to_string());
            Ok(())
        }
    }

    struct FailingPlugin;
    impl Plugin for FailingPlugin {
        fn name(&self) -> &str {
            "failing"
        }
        fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
            Err(EngineError::PluginRun {
                name: "failing".into(),
                message: "boom".into(),
            })
        }
    }

    #[test]
    fn register_and_run() {
        let mut engine = Engine::new();
        let runs = Arc::new(AtomicUsize::new(0));
        engine.register(CounterPlugin {
            name: "counter",
            runs: runs.clone(),
        });
        engine.init_all().unwrap();
        engine.run().unwrap();
        assert_eq!(engine.context().get("runs"), Some("1"));
        assert_eq!(runs.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn disabled_plugin_skipped() {
        let mut engine = Engine::new();
        let runs = Arc::new(AtomicUsize::new(0));
        engine.register(CounterPlugin {
            name: "counter",
            runs: runs.clone(),
        });
        engine.disable("counter");
        engine.init_all().unwrap();
        engine.run().unwrap();
        assert_eq!(runs.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn failing_plugin_propagates() {
        let mut engine = Engine::new();
        engine.register(FailingPlugin);
        engine.init_all().unwrap();
        assert!(engine.run().is_err());
    }

    #[test]
    fn run_before_init_errors() {
        let mut engine = Engine::new();
        assert!(matches!(engine.run(), Err(EngineError::NotInitialized)));
    }

    #[test]
    fn enable_unknown_errors() {
        let mut engine = Engine::new();
        assert!(engine.enable("nope").is_err());
    }

    struct TaggedPlugin {
        name: &'static str,
        tag: &'static str,
        ran: Arc<AtomicUsize>,
    }

    impl Plugin for TaggedPlugin {
        fn name(&self) -> &str {
            self.name
        }
        fn tags(&self) -> &[&str] {
            std::slice::from_ref(&self.tag)
        }
        fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
            self.ran.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[test]
    fn tag_selection_runs_only_matching() {
        let mut engine = Engine::new();
        let r1 = Arc::new(AtomicUsize::new(0));
        let r2 = Arc::new(AtomicUsize::new(0));
        engine.register(TaggedPlugin {
            name: "a",
            tag: "email",
            ran: r1.clone(),
        });
        engine.register(TaggedPlugin {
            name: "b",
            tag: "storage",
            ran: r2.clone(),
        });
        let selected: Vec<String> = engine
            .plugins_with_tag("email")
            .iter()
            .map(|s| s.to_string())
            .collect();
        engine.init_names(&selected);
        engine.run_names(&selected);
        assert_eq!(r1.load(Ordering::SeqCst), 1);
        assert_eq!(r2.load(Ordering::SeqCst), 0);
    }
}
