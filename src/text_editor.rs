use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    widgets::{Block, Paragraph},
    Frame,
};

pub struct TextEditor {
    pub cursor_position: u32,
    pub is_active: bool,
    pub text: String,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            cursor_position: 0,
            is_active: false,
            text: String::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn draw<'a>(&'a mut self, f: &mut Frame, area: Rect) {
        let mut block = Block::bordered();

        if self.is_active {
            block = block.border_style(Color::Blue);
        }

        let p = Paragraph::new(self.text.clone())
            .block(block)
            .style(Style::new().white().on_black());

        f.render_widget(p, area);
    }

    pub fn next_pos() {}

    pub fn prev_pos() {}

    pub fn line_up() {}

    pub fn line_down() {}

    pub fn delete() {}

    pub fn insert(char: char) {}
}
