use std::fmt;

#[derive(Debug)]
pub enum EngineError {
    PluginInit { name: String, message: String },
    PluginRun { name: String, message: String },
    PluginShutdown { name: String, message: String },
    NotFound(String),
    Disabled(String),
    NotInitialized,
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::PluginInit { name, message } => {
                write!(f, "plugin '{name}' failed to init: {message}")
            }
            EngineError::PluginRun { name, message } => {
                write!(f, "plugin '{name}' failed to run: {message}")
            }
            EngineError::PluginShutdown { name, message } => {
                write!(f, "plugin '{name}' failed to shut down: {message}")
            }
            EngineError::NotFound(name) => write!(f, "plugin '{name}' not found"),
            EngineError::Disabled(name) => write!(f, "plugin '{name}' is disabled"),
            EngineError::NotInitialized => write!(f, "engine not initialized"),
        }
    }
}

impl std::error::Error for EngineError {}
