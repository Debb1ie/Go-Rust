use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// A generic JSON API envelope
#[derive(Debug, Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data,
            timestamp: Utc::now(),
        }
    }
}

/// Server health/info
#[derive(Debug, Serialize)]
pub struct ServerInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub language: &'static str,
    pub framework: &'static str,
    pub uptime_hint: &'static str,
}

/// A project / portfolio item
#[derive(Debug, Serialize, Clone)]
pub struct Project {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub language: String,
    pub stars: u32,
    pub url: String,
}

/// Contact form payload
#[derive(Debug, Deserialize)]
pub struct ContactForm {
    pub name: String,
    pub email: String,
    pub message: String,
}

/// Contact form response
#[derive(Debug, Serialize)]
pub struct ContactAck {
    pub received: bool,
    pub reply_to: String,
}
