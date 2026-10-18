use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;
use serde_json::json;
use std::env;

pub struct TraccarSmsPlugin {
    pub url: String,
    pub to: String,
    pub message: String,
}

impl Plugin for TraccarSmsPlugin {
    fn name(&self) -> &str {
        "traccar_sms"
    }

    fn tags(&self) -> &[&str] {
        &["sms"]
    }

    fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        let token = env::var("API_TOKEN").map_err(|_| EngineError::PluginRun {
            name: "traccar_sms".into(),
            message: "missing env var API_TOKEN (Traccar SMS token)".into(),
        })?;

        let body = json!({
            "to": self.to,
            "message": self.message,
        });

        let resp = ureq::post(&self.url)
            .header("Authorization", &token)
            .header("Content-Type", "application/json")
            .send_json(body)
            .map_err(|e| EngineError::PluginRun {
                name: "traccar_sms".into(),
                message: format!("request failed: {e}"),
            })?;

        if !resp.status().is_success() {
            return Err(EngineError::PluginRun {
                name: "traccar_sms".into(),
                message: format!("traccar returned {}", resp.status()),
            });
        }

        println!("traccar sms sent to {}", self.to);
        Ok(())
    }
}
