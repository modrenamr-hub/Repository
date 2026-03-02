#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventKind {
    Character(char),
    Space,
    Enter,
    Punctuation(char),
    Modifier,
}

#[derive(Debug, Clone)]
pub struct KeyboardPolicy {
    pub min_word_len: usize,
}

impl Default for KeyboardPolicy {
    fn default() -> Self {
        Self { min_word_len: 3 }
    }
}

impl KeyboardPolicy {
    pub fn should_ignore_word(&self, word: &str, is_password_field: bool) -> bool {
        if is_password_field
            || word.chars().all(|c| c.is_numeric())
            || word.len() < self.min_word_len
        {
            return true;
        }
        false
    }
}
