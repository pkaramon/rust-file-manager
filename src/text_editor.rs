use std::{fs, path::PathBuf};

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
    editor::Editor,
    window::{Drawable, Focusable},
};

#[derive(Copy, Clone)]
struct CursorPosition {
    line: usize,
    char: usize,
}

pub struct TextEditor {
    cursor_position: CursorPosition,
    is_focused: bool,
    file: PathBuf,
    lines: Vec<String>,
}

impl TextEditor {
    pub fn new() -> Self {
        let editor = TextEditor {
            cursor_position: CursorPosition { line: 0, char: 0 },
            is_focused: false,
            file: PathBuf::new(),
            lines: Vec::new(),
        };
        editor
    }

    pub fn next_char(&mut self, _: KeyCode) -> bool {
        if self.lines.len() > 0 {
            let line = &self.lines[self.cursor_position.line];

            if self.cursor_position.char + 1 < line.len() {
                self.cursor_position.char += 1;
            }
        }
        true
    }

    pub fn prev_char(&mut self, _: KeyCode) -> bool {
        if self.lines.len() > 0 {
            if self.cursor_position.char > 0 {
                self.cursor_position.char -= 1;
            }
        }
        true
    }

    pub fn next_line(&mut self, _: KeyCode) -> bool {
        if self.cursor_position.line + 1 < self.lines.len() {
            self.cursor_position.line += 1;

            let line = &self.lines[self.cursor_position.line];
            if line.len() > 0 {
                if self.cursor_position.char > line.len() - 1 {
                    self.cursor_position.char = line.len() - 1;
                }
            } else {
                self.cursor_position.char = 0;
            }
        }
        true
    }

    pub fn prev_line(&mut self, _: KeyCode) -> bool {
        if self.cursor_position.line > 0 {
            self.cursor_position.line -= 1;

            let line = &self.lines[self.cursor_position.line];
            if line.len() > 0 {
                if self.cursor_position.char > line.len() - 1 {
                    self.cursor_position.char = line.len() - 1;
                }
            } else {
                self.cursor_position.char = 0;
            }
        }
        true
    }

    // pub fn delete() {}

    // pub fn insert(char: char) {}
    fn highlight_cursor<'a>(
        &'a self,
        (line_index, line_str): (usize, &'a str),
        cp: CursorPosition,
    ) -> Line {
        let cursor_line_index = cp.line;
        let char_index = cp.char;
        if cursor_line_index == line_index && char_index < line_str.len() && self.is_focused {
            let before = &line_str[..char_index];
            let highlighted = &line_str[char_index..char_index + 1];
            let after = &line_str[char_index + 1..];

            Line::from(vec![
                Span::from(before),
                Span::styled(
                    highlighted,
                    Style::default().fg(Color::Black).bg(Color::White),
                ),
                Span::from(after),
            ])
        } else {
            Line::from(line_str)
        }
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
            .map(|(index, line_str)| self.highlight_cursor((index, line_str), self.cursor_position))
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
    fn get_name(&self) -> &'static str {
        "text_editor"
    }
    fn get_commands(&self) -> Vec<Command<TextEditor>> {
        vec![
            Command {
                id: "text_editor.next_char",
                name: "Next char",
                func: TextEditor::next_char,
            },
            Command {
                id: "text_editor.prev_char",
                name: "Prev char",
                func: TextEditor::prev_char,
            },
            Command {
                id: "text_editor.next_line",
                name: "Next char",
                func: TextEditor::next_line,
            },
            Command {
                id: "text_editor.prev_line",
                name: "Prev line",
                func: TextEditor::prev_line,
            },
        ]
    }
}

impl Editor for TextEditor {
    fn set_path(&mut self, path: PathBuf) {
        self.file = path;
        let text =
            fs::read_to_string(self.file.clone()).unwrap_or("Unable to read file".to_string());
        self.lines = text.split("\n").map(|str| String::from(str)).collect();
    }
}
