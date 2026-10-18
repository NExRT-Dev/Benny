use super::context::Context;
use super::error::EngineError;

/// A feature that can be registered with the engine.
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;

    /// Tags/categories this plugin belongs to (e.g. "email", "storage").
    fn tags(&self) -> &[&str] {
        &[]
    }

    fn version(&self) -> &str {
        "0.1.0"
    }

    fn init(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        Ok(())
    }

    fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        Ok(())
    }

    fn shutdown(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        Ok(())
    }
}
