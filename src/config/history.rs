use serde::{Deserialize, Serialize};

static DEFAULT_SIZE: usize = 50;

fn default_size() -> usize {
    DEFAULT_SIZE
}

#[derive(Deserialize, Serialize)]
pub struct HistoryConfig {
    #[serde(default = "default_size")]
    size: usize,
}

impl HistoryConfig {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self { size: DEFAULT_SIZE }
    }
}
