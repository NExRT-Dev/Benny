use crate::core::context::Context;
use crate::core::error::EngineError;
use crate::core::plugin::Plugin;
use serde_json::json;
use std::env;

pub struct BrevoEmailPlugin {
    pub from_email: String,
    pub from_name: String,
    pub to_email: String,
    pub to_name: String,
    pub subject: String,
    pub html: String,
}

impl Plugin for BrevoEmailPlugin {
    fn name(&self) -> &str {
        "brevo_email"
    }

    fn tags(&self) -> &[&str] {
        &["email"]
    }

    fn run(&mut self, _ctx: &mut Context) -> Result<(), EngineError> {
        let api_key = env::var("API_KEY").map_err(|_| EngineError::PluginRun {
            name: "brevo_email".into(),
            message: "missing env var API_KEY (Brevo API key)".into(),
        })?;

        let body = json!({
            "sender": { "name": self.from_name, "email": self.from_email },
            "to": [{ "name": self.to_name, "email": self.to_email }],
            "subject": self.subject,
            "htmlContent": self.html,
        });

        let resp = ureq::post("https://api.brevo.com/v3/smtp/email")
            .header("api-key", &api_key)
            .send_json(body)
            .map_err(|e| EngineError::PluginRun {
                name: "brevo_email".into(),
                message: format!("request failed: {e}"),
            })?;

        if !resp.status().is_success() {
            return Err(EngineError::PluginRun {
                name: "brevo_email".into(),
                message: format!("brevo returned {}", resp.status()),
            });
        }

        println!("brevo email sent to {}", self.to_email);
        Ok(())
    }
}
