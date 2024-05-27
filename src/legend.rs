use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

pub struct Legend {}

impl Legend {
    pub fn draw<'a>(&'a mut self, f: &mut Frame, area: Rect) {
        let text = vec![Line::from(vec![
            Span::raw("First"),
            Span::styled("line", Style::new().green().italic()),
            ".".into(),
        ])];
        let p = Paragraph::new(text)
            .block(Block::bordered())
            .style(Style::new().white().on_black());

        f.render_widget(p, area);
    }
}
