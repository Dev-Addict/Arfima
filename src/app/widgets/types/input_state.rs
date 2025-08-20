#[derive(Debug, Default, Clone)]
pub struct InputState {
    buffer: String,
    cursor_position: usize,
}

impl InputState {
    pub fn new(buffer: &str) -> Self {
        Self {
            cursor_position: buffer.chars().count(),
            buffer: buffer.into(),
        }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn set_buffer(&mut self, buffer: String) {
        self.buffer = buffer;
        self.cursor_position = self.cursor_position.min(self.buffer.chars().count());
    }

    pub fn cursor_position(&self) -> usize {
        self.cursor_position
    }

    pub fn set_cursor_position(&mut self, cursor_position: usize) {
        self.cursor_position = cursor_position.min(self.buffer.chars().count());
    }

    pub fn insert_char(&mut self, c: char) {
        self.buffer.insert(self.get_byte_index(), c);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        if let Some((start, ch)) = self
            .buffer
            .char_indices()
            .nth(self.cursor_position.saturating_sub(1))
        {
            let end = start + ch.len_utf8();
            self.buffer.replace_range(start..end, "");
            self.cursor_position = self.cursor_position.saturating_sub(1);
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

    pub fn buffer_split(&self) -> (&str, &str) {
        self.buffer.split_at(self.get_byte_index())
    }

    fn get_byte_index(&self) -> usize {
        self.buffer
            .char_indices()
            .nth(self.cursor_position)
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.buffer.len())
    }
}
