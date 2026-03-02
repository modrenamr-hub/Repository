use musahih_pro::core::{
    buffer_manager::WordBufferManager,
    confidence_engine::CorrectionDecisionEngine,
    keyboard_listener::KeyEventKind,
    language_detector::{detect_script, LanguageDetector, ScriptHint},
    validator::{free_tier_limit_reached, ValidationConfig},
};

#[test]
fn buffer_flushes_on_space() {
    let mut buffer = WordBufferManager::new();
    buffer.push(KeyEventKind::Character('h'));
    buffer.push(KeyEventKind::Character('i'));
    let result = buffer.push(KeyEventKind::Space);
    assert_eq!(result, Some("hi".to_string()));
}

#[test]
fn detect_script_handles_arabic_and_latin() {
    assert_eq!(detect_script("hello"), ScriptHint::Latin);
    assert_eq!(detect_script("مرحبا"), ScriptHint::Arabic);
}

#[test]
fn decision_engine_respects_threshold() {
    let detector = LanguageDetector::default();
    let result = detector.analyze("ghbdk");
    let engine = CorrectionDecisionEngine::new(50);
    assert!(engine.decide(&result).is_some());
}

#[test]
fn free_tier_limit_is_enforced() {
    let cfg = ValidationConfig::default();
    assert!(free_tier_limit_reached(cfg.daily_limit_free, &cfg, false));
    assert!(!free_tier_limit_reached(1, &cfg, true));
}
