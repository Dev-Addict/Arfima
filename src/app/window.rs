use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub trait Window {
    fn render(&self, frame: &mut Frame, area: Rect);
}

pub struct Split {
    direction: Direction,
    windows: Vec<Box<dyn Window>>,
}

impl Split {
    pub fn new(direction: Direction, windows: Vec<Box<dyn Window>>) -> Self {
        Self { direction, windows }
    }
}

impl Window for Split {
    fn render(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(self.direction)
            .constraints(vec![Constraint::Fill(1); self.windows.len()])
            .split(area);

        for (i, window) in self.windows.iter().enumerate() {
            window.render(frame, layout[i]);
        }
    }
}

pub struct DefaultWindow {
    render_fn: fn(frame: &mut Frame, area: Rect),
}

impl DefaultWindow {
    pub fn new(render_fn: fn(frame: &mut Frame, area: Rect)) -> Self {
        Self { render_fn }
    }
}

impl Window for DefaultWindow {
    fn render(&self, frame: &mut Frame, area: Rect) {
        (self.render_fn)(frame, area);
    }
}
