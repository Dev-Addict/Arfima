#[derive(Debug, Default, Clone)]
pub enum InputStateMode {
    #[default]
    Normal,
    Select {
        origin: usize,
    },
}

#[derive(Debug, Default, Clone)]
pub struct InputState {
    mode: InputStateMode,
    buffer: String,
    cursor_position: usize,
}

impl InputState {
    pub fn new(buffer: &str) -> Self {
        Self {
            mode: InputStateMode::Normal,
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

    pub fn mode(&self) -> &InputStateMode {
        &self.mode
    }

    fn remove_selection(&mut self) {
        if let InputStateMode::Select { origin } = self.mode {
            if self.cursor_position < origin {
                if let Some((start, _)) = self.buffer.char_indices().nth(self.cursor_position) {
                    if let Some((ch_end, ch)) = self
                        .buffer
                        .char_indices()
                        .nth(origin.min(self.buffer.chars().count().saturating_sub(1)))
                    {
                        let end = ch_end + ch.len_utf8();
                        self.buffer.replace_range(start..end, "");
                        self.cursor_position =
                            self.cursor_position.min(self.buffer.chars().count());
                    }
                }
            } else if let Some((start, _)) = self
                .buffer
                .char_indices()
                .nth(origin.min(self.cursor_position))
            {
                if let Some((ch_end, ch)) = self.buffer.char_indices().nth(
                    self.cursor_position
                        .min(self.buffer.chars().count().saturating_sub(1)),
                ) {
                    let end = ch_end + ch.len_utf8();
                    self.buffer.replace_range(start..end, "");
                    self.cursor_position = origin.min(self.buffer.chars().count());
                }
            }

            self.mode = InputStateMode::Normal;
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.remove_selection();
        self.buffer
            .insert(self.get_byte_index(self.cursor_position), c);
        self.cursor_position += 1;
    }

    pub fn remove_char(&mut self) {
        match self.mode {
            InputStateMode::Normal => {
                if self.cursor_position == 0 {
                    return;
                }

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
            InputStateMode::Select { .. } => self.remove_selection(),
        }
    }

    fn premove(&mut self, select: bool) {
        match self.mode {
            InputStateMode::Normal => {
                if select {
                    self.mode = InputStateMode::Select {
                        origin: self.cursor_position,
                    };
                }
            }
            InputStateMode::Select { .. } => {
                if !select {
                    self.mode = InputStateMode::Normal;
                }
            }
        }
    }

    pub fn left(&mut self, select: bool) {
        self.premove(select);

        self.cursor_position = self.cursor_position.saturating_sub(1);
    }

    pub fn right(&mut self, select: bool) {
        self.premove(select);

        self.cursor_position = self
            .cursor_position
            .saturating_add(1)
            .min(self.buffer.len());
    }

    pub fn buffer_split(&self) -> Vec<&str> {
        match self.mode {
            InputStateMode::Normal => {
                let (l, r) = self
                    .buffer
                    .split_at(self.get_byte_index(self.cursor_position));
                vec![l, r]
            }
            InputStateMode::Select { origin } => {
                let (smaller, bigger) = if origin > self.cursor_position {
                    (
                        self.cursor_position,
                        (origin + 1).min(self.buffer.chars().count()),
                    )
                } else {
                    (origin, self.cursor_position)
                };
                let (l, r) = self.buffer.split_at(self.get_byte_index(smaller));
                let (m, r) = r.split_at(self.get_byte_index(bigger) - self.get_byte_index(smaller));

                vec![l, m, r]
            }
        }
    }

    fn get_byte_index(&self, position: usize) -> usize {
        self.buffer
            .char_indices()
            .nth(position)
            .map(|(i, _)| i)
            .unwrap_or_else(|| self.buffer.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let state = InputState::new("");
        assert_eq!(state.buffer(), "");
        assert_eq!(state.cursor_position(), 0);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_new_with_text() {
        let state = InputState::new("hello");
        assert_eq!(state.buffer(), "hello");
        assert_eq!(state.cursor_position(), 5);
    }

    #[test]
    fn test_new_with_unicode() {
        let state = InputState::new("héllo 🌟");
        assert_eq!(state.buffer(), "héllo 🌟");
        assert_eq!(state.cursor_position(), 7);
    }

    #[test]
    fn test_default() {
        let state = InputState::default();
        assert_eq!(state.buffer(), "");
        assert_eq!(state.cursor_position(), 0);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_set_buffer() {
        let mut state = InputState::new("hello");
        state.set_buffer("world".to_string());
        assert_eq!(state.buffer(), "world");
        assert_eq!(state.cursor_position(), 5);
    }

    #[test]
    fn test_set_buffer_cursor_adjustment() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(10);
        state.set_buffer("hi".to_string());
        assert_eq!(state.buffer(), "hi");
        assert_eq!(state.cursor_position(), 2);
    }

    #[test]
    fn test_set_cursor_position() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(3);
        assert_eq!(state.cursor_position(), 3);
    }

    #[test]
    fn test_set_cursor_position_out_of_bounds() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(10);
        assert_eq!(state.cursor_position(), 5);
    }

    #[test]
    fn test_insert_char_normal_mode() {
        let mut state = InputState::new("hllo");
        state.set_cursor_position(1);
        state.insert_char('e');
        assert_eq!(state.buffer(), "hello");
        assert_eq!(state.cursor_position(), 2);
    }

    #[test]
    fn test_insert_char_at_end() {
        let mut state = InputState::new("hello");
        state.insert_char('!');
        assert_eq!(state.buffer(), "hello!");
        assert_eq!(state.cursor_position(), 6);
    }

    #[test]
    fn test_insert_char_unicode() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.insert_char('🌟');
        assert_eq!(state.buffer(), "🌟hello");
        assert_eq!(state.cursor_position(), 1);
    }

    #[test]
    fn test_remove_char_normal_mode() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(5);
        state.remove_char();
        assert_eq!(state.buffer(), "hell");
        assert_eq!(state.cursor_position(), 4);
    }

    #[test]
    fn test_remove_char_at_beginning() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.remove_char();
        assert_eq!(state.buffer(), "hello");
        assert_eq!(state.cursor_position(), 0);
    }

    #[test]
    fn test_remove_char_unicode() {
        let mut state = InputState::new("🌟hello");
        state.set_cursor_position(1);
        state.remove_char();
        assert_eq!(state.buffer(), "hello");
        assert_eq!(state.cursor_position(), 0);
    }

    #[test]
    fn test_left_movement_normal() {
        let mut state = InputState::new("hello");
        state.left(false);
        assert_eq!(state.cursor_position(), 4);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_left_movement_at_beginning() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.left(false);
        assert_eq!(state.cursor_position(), 0);
    }

    #[test]
    fn test_left_with_selection_start() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(3);
        state.left(true);
        assert_eq!(state.cursor_position(), 2);
        match state.mode {
            InputStateMode::Select { origin } => assert_eq!(origin, 3),
            _ => panic!("Expected Select mode"),
        }
    }

    #[test]
    fn test_left_with_selection_continue() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(3);
        state.left(true);
        state.left(true);
        assert_eq!(state.cursor_position(), 1);
        match state.mode {
            InputStateMode::Select { origin } => assert_eq!(origin, 3),
            _ => panic!("Expected Select mode"),
        }
    }

    #[test]
    fn test_left_cancel_selection() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(3);
        state.left(true);
        state.left(false);
        assert_eq!(state.cursor_position(), 1);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_right_movement_normal() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.right(false);
        assert_eq!(state.cursor_position(), 1);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_right_movement_at_end() {
        let mut state = InputState::new("hello");
        state.right(false);
        assert_eq!(state.cursor_position(), 5);
    }

    #[test]
    fn test_right_with_selection() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(2);
        state.right(true);
        assert_eq!(state.cursor_position(), 3);
        match state.mode {
            InputStateMode::Select { origin } => assert_eq!(origin, 2),
            _ => panic!("Expected Select mode"),
        }
    }

    #[test]
    fn test_get_byte_index_positions() {
        let state = InputState::new("hello world");
        assert_eq!(state.get_byte_index(0), 0);
        assert_eq!(state.get_byte_index(5), 5);
        assert_eq!(state.get_byte_index(11), 11);
        assert_eq!(state.get_byte_index(20), 11);
    }

    #[test]
    fn test_get_byte_index_unicode() {
        let state = InputState::new("hé🌟lo");
        assert_eq!(state.get_byte_index(0), 0);
        assert_eq!(state.get_byte_index(1), 1);
        assert_eq!(state.get_byte_index(2), 3);
        assert_eq!(state.get_byte_index(3), 7);
        assert_eq!(state.get_byte_index(4), 8);
        assert_eq!(state.get_byte_index(5), 9);
        assert_eq!(state.get_byte_index(10), 9);
    }

    #[test]
    fn test_get_byte_index_empty_buffer() {
        let state = InputState::new("");
        assert_eq!(state.get_byte_index(0), 0);
        assert_eq!(state.get_byte_index(5), 0);
    }

    #[test]
    fn test_buffer_split_normal_mode_beginning() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(0);
        let parts = state.buffer_split();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], "hello world");
    }

    #[test]
    fn test_buffer_split_normal_mode_end() {
        let state = InputState::new("hello world");
        let parts = state.buffer_split();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "hello world");
        assert_eq!(parts[1], "");
    }

    #[test]
    fn test_buffer_split_normal_mode_middle() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(6);
        let parts = state.buffer_split();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "hello ");
        assert_eq!(parts[1], "world");
    }

    #[test]
    fn test_buffer_split_select_mode_forward_selection() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(2);
        state.right(true);
        state.right(true);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "he");
        assert_eq!(parts[1], "llo");
        assert_eq!(parts[2], " world");
    }

    #[test]
    fn test_buffer_split_select_mode_backward_selection() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(5);
        state.left(true);
        state.left(true);
        state.left(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "he");
        assert_eq!(parts[1], "llo ");
        assert_eq!(parts[2], "world");
    }

    #[test]
    fn test_buffer_split_select_mode_single_char() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(2);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "he");
        assert_eq!(parts[1], "l");
        assert_eq!(parts[2], "lo");
    }

    #[test]
    fn test_buffer_split_select_mode_at_beginning() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.right(true);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], "he");
        assert_eq!(parts[2], "llo");
    }

    #[test]
    fn test_buffer_split_select_mode_at_end() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(3);
        state.right(true);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "hel");
        assert_eq!(parts[1], "lo");
        assert_eq!(parts[2], "");
    }

    #[test]
    fn test_buffer_split_select_mode_unicode() {
        let mut state = InputState::new("hé🌟lo");
        state.set_cursor_position(1);
        state.right(true);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "h");
        assert_eq!(parts[1], "é🌟");
        assert_eq!(parts[2], "lo");
    }

    #[test]
    fn test_buffer_split_select_mode_whole_buffer() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(0);
        state.right(true);
        state.right(true);
        state.right(true);
        state.right(true);
        state.right(true);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], "hello");
        assert_eq!(parts[2], "");
    }

    #[test]
    fn test_buffer_split_select_mode_origin_beyond_buffer() {
        let mut state = InputState::new("abc");

        state.mode = InputStateMode::Select { origin: 10 };
        state.set_cursor_position(1);

        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "a");
        assert_eq!(parts[1], "bc");
        assert_eq!(parts[2], "");
    }

    #[test]
    fn test_buffer_split_empty_buffer_normal() {
        let state = InputState::new("");
        let parts = state.buffer_split();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], "");
    }

    #[test]
    fn test_buffer_split_empty_buffer_select() {
        let mut state = InputState::new("");
        state.mode = InputStateMode::Select { origin: 0 };
        let parts = state.buffer_split();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "");
        assert_eq!(parts[1], "");
        assert_eq!(parts[2], "");
    }

    #[test]
    fn test_insert_char_uses_get_byte_index() {
        let mut state = InputState::new("hé🌟lo");
        state.set_cursor_position(2);
        state.insert_char('X');
        assert_eq!(state.buffer(), "héX🌟lo");
        assert_eq!(state.cursor_position(), 3);
    }

    #[test]
    fn test_insert_char_removes_selection() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(0);
        state.right(true);
        state.right(true);
        state.insert_char('H');
        assert_eq!(state.buffer(), "Hlo world");
        assert_eq!(state.cursor_position(), 1);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_remove_selection_cursor_before_origin() {
        let mut state = InputState::new("hello world");
        state.set_cursor_position(6);
        state.right(true);
        state.right(true);
        state.left(false);
        state.remove_char();
        assert_eq!(state.buffer(), "hello orld");
    }

    #[test]
    fn test_remove_selection_cursor_after_origin() {
        let mut state = InputState::new("hello");
        state.set_cursor_position(2);
        state.right(true);
        state.remove_char();
        assert_eq!(state.buffer(), "heo");
        assert_eq!(state.cursor_position(), 2);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_complex_selection_scenario() {
        let mut state = InputState::new("abcdefg");
        state.set_cursor_position(2);
        state.right(true);
        state.right(true);
        state.right(true);

        state.insert_char('X');
        assert_eq!(state.buffer(), "abXg");
        assert_eq!(state.cursor_position(), 3);
        assert!(matches!(state.mode, InputStateMode::Normal));
    }

    #[test]
    fn test_empty_buffer_operations() {
        let mut state = InputState::new("");
        state.left(false);
        assert_eq!(state.cursor_position(), 0);

        state.right(false);
        assert_eq!(state.cursor_position(), 0);

        state.remove_char();
        assert_eq!(state.buffer(), "");

        state.insert_char('a');
        assert_eq!(state.buffer(), "a");
        assert_eq!(state.cursor_position(), 1);
    }
}
