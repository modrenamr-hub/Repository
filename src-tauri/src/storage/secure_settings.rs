use crate::core::validator::ValidationConfig;
use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionSettings {
    pub confidence_threshold: u8,
    pub sensitivity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseSettings {
    pub key: Option<String>,
    pub plan: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub detection: DetectionSettings,
    pub validation: ValidationConfig,
    pub license: LicenseSettings,
    pub ignored_words: Vec<String>,
    pub run_on_startup: bool,
    pub dark_mode: bool,
    pub arabic_first_ui: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            detection: DetectionSettings {
                confidence_threshold: 70,
                sensitivity: 65,
            },
            validation: ValidationConfig::default(),
            license: LicenseSettings {
                key: None,
                plan: "free".to_string(),
            },
            ignored_words: Vec::new(),
            run_on_startup: true,
            dark_mode: true,
            arabic_first_ui: true,
        }
    }
}

pub struct SecureSettingsStore {
    path: PathBuf,
    settings: AppSettings,
}

impl SecureSettingsStore {
    pub fn load_or_default(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path = path.into();
        let settings = if path.exists() {
            let raw = fs::read_to_string(&path).context("failed to read settings")?;
            serde_json::from_str(&raw).context("failed to parse settings")?
        } else {
            AppSettings::default()
        };

        Ok(Self { path, settings })
    }

    pub fn snapshot(&self) -> AppSettings {
        self.settings.clone()
    }

    pub fn import_from_file(&mut self, source: impl Into<PathBuf>) -> anyhow::Result<()> {
        let source = source.into();
        let raw = fs::read_to_string(source).context("failed to read import settings")?;
        self.settings = serde_json::from_str(&raw).context("failed to decode imported settings")?;
        self.save()
    }

    pub fn export_to_file(&self, destination: impl Into<PathBuf>) -> anyhow::Result<()> {
        let destination = destination.into();
        let payload = serde_json::to_string_pretty(&self.settings)?;
        fs::write(destination, payload).context("failed to export settings")?;
        Ok(())
    }
    pub fn save(&self) -> anyhow::Result<()> {
        let payload = serde_json::to_string_pretty(&self.settings)?;
        fs::write(&self.path, payload).context("failed to persist settings")?;
        Ok(())
    }
}
