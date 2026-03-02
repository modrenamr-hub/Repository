use super::language_detector::{DetectionResult, ScriptHint};

#[derive(Debug, Clone)]
pub struct CorrectionSuggestion {
    pub from: String,
    pub to: String,
    pub confidence: u8,
}

#[derive(Debug)]
pub struct CorrectionDecisionEngine {
    threshold: u8,
}

impl CorrectionDecisionEngine {
    pub fn new(threshold: u8) -> Self {
        Self { threshold }
    }

    pub fn decide(&self, result: &DetectionResult) -> Option<CorrectionSuggestion> {
        if matches!(result.hint, ScriptHint::Mixed | ScriptHint::Unknown) {
            return None;
        }

        if result.confidence >= self.threshold {
            return Some(CorrectionSuggestion {
                from: result.original.clone(),
                to: result.converted.clone(),
                confidence: result.confidence,
            });
        }
        None
    }
}
