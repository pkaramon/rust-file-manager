use ratatui::{layout::Rect, Frame};

pub trait Window: Drawable + Focusable {}

pub trait Drawable {
    fn draw(&mut self, f: &mut Frame, area: Rect);
}

pub trait Focusable {
    fn focus(&mut self);
    fn unfocus(&mut self);
    fn is_focused(&self) -> bool;
}
