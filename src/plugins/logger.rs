use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;

pub struct LoggerPlugin {
    pub message: String,
}

impl Plugin for LoggerPlugin {
    fn name(&self) -> &str {
        "logger"
    }

    fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        println!("{}", self.message);
        Ok(())
    }
}
