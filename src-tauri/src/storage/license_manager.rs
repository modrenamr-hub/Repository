use super::secure_settings::LicenseSettings;
use anyhow::Context;
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseToken {
    pub key_hash: String,
    pub device_id: String,
    pub expires_at: DateTime<Utc>,
    pub plan: String,
}

pub struct LicenseManager {
    settings: LicenseSettings,
}

impl LicenseManager {
    pub fn new(settings: LicenseSettings) -> anyhow::Result<Self> {
        Ok(Self { settings })
    }

    pub fn create_device_bound_hash(license_key: &str, device_id: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(license_key.as_bytes());
        hasher.update(device_id.as_bytes());
        STANDARD.encode(hasher.finalize())
    }

    pub fn offline_grace_valid(last_verified: DateTime<Utc>) -> bool {
        Utc::now() - last_verified <= Duration::days(7)
    }

    pub fn current_plan(&self) -> &str {
        &self.settings.plan
    }

    pub fn api_endpoint() -> anyhow::Result<String> {
        std::env::var("MUSAHIH_LICENSE_API").context("MUSAHIH_LICENSE_API missing")
    }
}
