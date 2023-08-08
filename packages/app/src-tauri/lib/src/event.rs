use serde::{Deserialize, Serialize};
use tauri::Window;
use tracing::instrument;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum Event {
    #[serde(rename = "toast")]
    #[serde(alias = "toast")]
    Toast {
        variant: ToastVariant,
        message: String,
    },
    #[serde(rename = "auth-url")]
    #[serde(alias = "auth-url")]
    AuthUrl { url: String },
}

impl Event {
    #[instrument(skip(window))]
    pub fn emit(&self, window: &Window) {
        window.emit(&self.name(), &self).unwrap();
    }

    pub fn name(&self) -> &str {
        match self {
            Event::Toast {
                variant: _,
                message: _,
            } => "toast",
            Event::AuthUrl { url: _ } => "auth-url",
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ToastVariant {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "neutral")]
    Neutral,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "danger")]
    Danger,
}
