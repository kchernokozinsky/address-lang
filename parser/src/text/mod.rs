pub mod span;

pub struct SourceText {
    text: String,
}

impl SourceText {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }

    pub fn get_line(&self, index: usize) -> &str {
        self.text.lines().nth(index).unwrap()
    }
}

