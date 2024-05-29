use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph},
    Frame,
};

use crate::{
    command::{Command, CommandHandler},
    window::{Drawable, Focusable},
};

#[derive(Copy, Clone)]
struct CursorPosition {
    line: usize,
    char: usize,
}

pub struct TextEditor {
    cursor_position: CursorPosition,
    pub is_focused: bool,
    lines: Vec<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        TextEditor {
            cursor_position: CursorPosition { line: 0, char: 0 },
            is_focused: false,
            lines: Vec::new(),
        }
    }

    pub fn set_text(&mut self, text: String) {
        self.lines = text.split("\n").map(|str| String::from(str)).collect();
    }

    // fn log(&self) -> std::io::Result<()> {
    //     let mut file = OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open("log.txt")?;
    //     file.write_all(format!("{}", self.lines.len()).as_bytes())
    // }

    pub fn next_char(&mut self, _: KeyCode) -> bool {
        if self.lines.len() > 0 {
            let line = &self.lines[self.cursor_position.line];

            if self.cursor_position.char + 1 < line.len() {
                self.cursor_position.char += 1;
            }
        }
        true
    }

    // pub fn prev_char() {}

    // pub fn next_line() {}

    // pub fn prev_line() {}

    // pub fn delete() {}

    // pub fn insert(char: char) {}

    // fn go_back(&mut self, _: KeyCode) {}
}

fn highlight_cursor((line_index, line_str): (usize, &str), cp: CursorPosition) -> Line {
    let cursor_line_index = cp.line;
    let char_index = cp.char;
    if cursor_line_index == line_index && char_index < line_str.len() {
        let before = &line_str[..char_index];
        let highlighted = &line_str[char_index..char_index + 1];
        let after = &line_str[char_index + 1..];

        Line::from(vec![
            Span::from(before),
            Span::styled(highlighted, Color::Red),
            Span::from(after),
        ])
    } else {
        Line::from(line_str)
    }
}

impl Drawable for TextEditor {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        let mut block = Block::bordered();

        if self.is_focused {
            block = block.border_style(Color::Blue);
        }

        let lines: Vec<Line> = self
            .lines
            .iter()
            .enumerate()
            .map(|(index, line_str)| highlight_cursor((index, line_str), self.cursor_position))
            .collect();

        let p = Paragraph::new(lines)
            .block(block)
            .style(Style::new().white().on_black());

        f.render_widget(Clear, area);
        f.render_widget(p, area);
    }
}

impl Focusable for TextEditor {
    fn focus(&mut self) {
        self.is_focused = true;
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }

    fn is_focused(&self) -> bool {
        self.is_focused
    }
}

impl CommandHandler for TextEditor {
    fn get_name(&mut self) -> &'static str {
        "text_editor"
    }
    fn get_commands(&self) -> Vec<Command<Self>> {
        vec![Command {
            id: "text_editor.next_char",
            name: "Next char",
            func: TextEditor::next_char,
        }]
    }
}
