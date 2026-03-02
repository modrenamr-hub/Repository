use anyhow::Context;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationRequest {
    pub license_key: String,
    pub device_id: String,
    pub app_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResponse {
    pub valid: bool,
    pub plan: String,
    pub expires_at: Option<String>,
}

pub struct SubscriptionValidator {
    client: reqwest::Client,
}

impl SubscriptionValidator {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn verify(&self, payload: &ValidationRequest) -> anyhow::Result<ValidationResponse> {
        let url = std::env::var("MUSAHIH_LICENSE_API").context("MUSAHIH_LICENSE_API missing")?;
        let response = self.client.post(url).json(payload).send().await?;
        Ok(response.json::<ValidationResponse>().await?)
    }
}
