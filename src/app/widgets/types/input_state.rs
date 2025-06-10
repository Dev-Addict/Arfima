#[derive(Debug, Default)]
pub struct InputState {
    buffer: String,
    cursor_position: usize,
}

impl InputState {
    pub fn new(buffer: &str, cursor_position: usize) -> Self {
        Self {
            buffer: buffer.into(),
            cursor_position,
        }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, cursor_position: usize) {
        self.cursor_position = cursor_position;
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.cursor_position, c);
        self.cursor_position = self.cursor_position.saturating_add(1);
    }

    pub fn remove_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position = self.cursor_position.saturating_sub(1);
            self.buffer.remove(self.cursor_position);
        }
    }

    pub fn left(&mut self) {
        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub fn right(&mut self) {
        self.cursor_position = self
            .cursor_position
            .saturating_add(1)
            .min(self.buffer.len());
    }
}
