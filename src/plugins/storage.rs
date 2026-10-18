use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;

/// Example feature plugin: stores a value into context.
pub struct StoragePlugin {
    pub key: String,
    pub value: String,
}

impl Plugin for StoragePlugin {
    fn name(&self) -> &str {
        "storage"
    }

    fn tags(&self) -> &[&str] {
        &["storage"]
    }

    fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        ctx.set(&self.key, &self.value);
        println!("stored {} = {}", self.key, self.value);
        Ok(())
    }
}
