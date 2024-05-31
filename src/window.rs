use ratatui::{layout::Rect, Frame};

pub trait Drawable {
    fn draw(&self, f: &mut Frame, area: Rect);
}

pub trait Focusable {
    fn focus(&mut self);
    fn unfocus(&mut self);
    fn is_focused(&self) -> bool;
}
