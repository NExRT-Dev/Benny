use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;

pub struct PrinterPlugin {
    pub key: String,
}

impl Plugin for PrinterPlugin {
    fn name(&self) -> &str {
        "printer"
    }

    fn run(&mut self, ctx: &mut Context) -> Result<(), EngineError> {
        match ctx.get(&self.key) {
            Some(v) => println!("{} = {}", self.key, v),
            None => println!("{} not set", self.key),
        }
        Ok(())
    }
}
