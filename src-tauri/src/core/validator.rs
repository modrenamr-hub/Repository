use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CorrectionMode {
    SuggestOnly,
    AutoInstant,
    AutoSilent,
    ManualHotkey,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    pub mode: CorrectionMode,
    pub daily_limit_free: u32,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        Self {
            mode: CorrectionMode::SuggestOnly,
            daily_limit_free: 120,
        }
    }
}

pub fn free_tier_limit_reached(current_count: u32, cfg: &ValidationConfig, is_pro: bool) -> bool {
    !is_pro && current_count >= cfg.daily_limit_free
}
