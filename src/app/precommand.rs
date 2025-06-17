#[derive(Debug)]
pub enum Precommand {
    Leader,
    Repeat(usize),
    Window,
}
