pub mod email;
pub mod logger;
pub mod printer;
pub mod storage;
pub mod traccar_sms;

pub use email::BrevoEmailPlugin;
pub use logger::LoggerPlugin;
pub use printer::PrinterPlugin;
pub use storage::StoragePlugin;
pub use traccar_sms::TraccarSmsPlugin;

use crate::engine::Engine;

/// Helper to register a batch of plugins.
pub fn register_all(engine: &mut Engine) {
    engine.register(LoggerPlugin {
        message: "logger plugin running".to_string(),
    });
    engine.register(PrinterPlugin {
        key: "count".to_string(),
    });
    engine.register(BrevoEmailPlugin {
        from_email: "sender@example.com".to_string(),
        from_name: "Benny".to_string(),
        to_email: "user@example.com".to_string(),
        to_name: "User".to_string(),
        subject: "Hello from Benny".to_string(),
        html: "<p>Hello from Benny</p>".to_string(),
    });
    engine.register(StoragePlugin {
        key: "greeting".to_string(),
        value: "stored hello".to_string(),
    });
    engine.register(TraccarSmsPlugin {
        url: "http://127.0.0.1:8082".to_string(),
        to: "+1234567890".to_string(),
        message: "Hello from Benny".to_string(),
    });
}
