#[derive(Debug)]
pub enum Precommand {
    Leader,
    Repeat(usize),
    RepeatWindow(usize),
}
