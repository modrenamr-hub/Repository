use super::converter::LayoutConverter;
use strsim::levenshtein;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptHint {
    Latin,
    Arabic,
    Mixed,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub original: String,
    pub converted: String,
    pub hint: ScriptHint,
    pub confidence: u8,
}

#[derive(Debug)]
pub struct LanguageDetector {
    converter: LayoutConverter,
    latin_dictionary: Vec<&'static str>,
    arabic_dictionary: Vec<&'static str>,
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self {
            converter: LayoutConverter::default(),
            latin_dictionary: vec!["hello", "keyboard", "project", "premium", "update"],
            arabic_dictionary: vec!["مرحبا", "لوحة", "مشروع", "تصحيح", "تحديث"],
        }
    }
}

impl LanguageDetector {
    pub fn analyze(&self, word: &str) -> DetectionResult {
        let hint = detect_script(word);
        let converted = match hint {
            ScriptHint::Latin => self.converter.convert_en_to_ar(word),
            ScriptHint::Arabic => self.converter.convert_ar_to_en(word),
            _ => word.to_string(),
        };
        let dict_score = self.dictionary_score(word, &converted, hint);
        let pattern_score = match hint {
            ScriptHint::Latin | ScriptHint::Arabic => 35,
            ScriptHint::Mixed => 10,
            ScriptHint::Unknown => 0,
        };
        let confidence = (dict_score + pattern_score).min(100);

        DetectionResult {
            original: word.to_string(),
            converted,
            hint,
            confidence: confidence as u8,
        }
    }

    fn dictionary_score(&self, original: &str, converted: &str, hint: ScriptHint) -> i32 {
        let target = match hint {
            ScriptHint::Latin => &self.arabic_dictionary,
            ScriptHint::Arabic => &self.latin_dictionary,
            _ => return 0,
        };

        target
            .iter()
            .map(|candidate| {
                let dist = levenshtein(converted, candidate);
                let base = 65_i32.saturating_sub((dist as i32) * 12);
                if original.len() > 4 {
                    base + 8
                } else {
                    base
                }
            })
            .max()
            .unwrap_or(0)
    }
}

pub fn detect_script(word: &str) -> ScriptHint {
    let mut latin = false;
    let mut arabic = false;

    for c in word.chars() {
        if c.is_ascii_alphabetic() {
            latin = true;
        } else if ('\u{0600}'..='\u{06FF}').contains(&c) {
            arabic = true;
        }
    }

    match (latin, arabic) {
        (true, false) => ScriptHint::Latin,
        (false, true) => ScriptHint::Arabic,
        (true, true) => ScriptHint::Mixed,
        _ => ScriptHint::Unknown,
    }
}
