use ahash::AHashMap;

#[derive(Default)]
pub struct AdaptiveLearningModel {
    correction_memory: AHashMap<String, String>,
}

impl AdaptiveLearningModel {
    pub fn remember(&mut self, from: &str, to: &str) {
        self.correction_memory
            .insert(from.to_string(), to.to_string());
    }

    pub fn predict(&self, word: &str) -> Option<String> {
        self.correction_memory.get(word).cloned()
    }
}
