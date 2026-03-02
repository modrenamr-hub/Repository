use anyhow::Context;

pub struct UpdateChecker;

impl UpdateChecker {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_latest_version(&self) -> anyhow::Result<String> {
        let endpoint =
            std::env::var("MUSAHIH_UPDATE_ENDPOINT").context("missing update endpoint")?;
        let body = reqwest::get(endpoint).await?.text().await?;
        Ok(body)
    }
}
