use crate::core::{
    buffer_manager::WordBufferManager, confidence_engine::CorrectionDecisionEngine,
    language_detector::LanguageDetector,
};
use crate::services::{
    subscription_validator::SubscriptionValidator, update_checker::UpdateChecker,
};
use crate::storage::{license_manager::LicenseManager, secure_settings::SecureSettingsStore};
use crate::ui::{
    notification::NotificationController, settings::SettingsController, tray::TrayController,
};
use anyhow::Context;

pub struct MusahihApp {
    detector: LanguageDetector,
    decision_engine: CorrectionDecisionEngine,
    _buffer: WordBufferManager,
    _settings: SecureSettingsStore,
    _license: LicenseManager,
    _notifier: NotificationController,
    _settings_ui: SettingsController,
    _tray: TrayController,
    _updates: UpdateChecker,
    _subscription: SubscriptionValidator,
}

impl MusahihApp {
    pub async fn bootstrap() -> anyhow::Result<Self> {
        let settings = SecureSettingsStore::load_or_default("musahih-settings.json")?;
        let license = LicenseManager::new(settings.snapshot().license.clone())?;

        Ok(Self {
            detector: LanguageDetector::default(),
            decision_engine: CorrectionDecisionEngine::new(
                settings.snapshot().detection.confidence_threshold,
            ),
            _buffer: WordBufferManager::new(),
            _settings: settings,
            _license: license,
            _notifier: NotificationController::new(),
            _settings_ui: SettingsController::new(),
            _tray: TrayController::new(),
            _updates: UpdateChecker::new(),
            _subscription: SubscriptionValidator::new(),
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        let sample = "ghbdk";
        let analysis = self.detector.analyze(sample);
        if let Some(suggestion) = self.decision_engine.decide(&analysis) {
            tracing::info!(?suggestion, "sample correction decision");
        }

        tracing::info!("Musahih Pro initialized and running");
        Ok(()).context("application runtime failed")
    }
}
