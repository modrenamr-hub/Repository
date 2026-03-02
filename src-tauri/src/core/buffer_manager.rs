use super::keyboard_listener::KeyEventKind;

#[derive(Default)]
pub struct WordBufferManager {
    current: String,
}

impl WordBufferManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, event: KeyEventKind) -> Option<String> {
        match event {
            KeyEventKind::Character(c) => {
                self.current.push(c);
                None
            }
            KeyEventKind::Space | KeyEventKind::Enter | KeyEventKind::Punctuation(_) => {
                self.flush()
            }
            KeyEventKind::Modifier => None,
        }
    }

    pub fn flush(&mut self) -> Option<String> {
        if self.current.is_empty() {
            return None;
        }
        Some(std::mem::take(&mut self.current))
    }
}
