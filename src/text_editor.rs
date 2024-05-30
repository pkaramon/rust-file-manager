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
    as_command,
    command::{Command, CommandHandler, InputHandler},
    editor::Editor,
    window::{Drawable, Focusable},
};

#[derive(Copy, Clone)]
struct CursorPosition {
    line: usize,
    char: usize,
}

impl CursorPosition {
    fn new() -> Self {
        CursorPosition { line: 0, char: 0 }
    }
}

#[derive(PartialEq)]
enum Mode {
    View,
    Edit,
}

pub struct TextEditor {
    cursor_position: CursorPosition,
    is_focused: bool,
    file: PathBuf,
    lines: Vec<String>,
    mode: Mode,
    file_saved: bool,
}

impl TextEditor {
    pub fn new() -> Self {
        let editor = TextEditor {
            cursor_position: CursorPosition { line: 0, char: 0 },
            is_focused: false,
            file: PathBuf::new(),
            lines: Vec::new(),
            mode: Mode::View,
            file_saved: true,
        };
        editor
    }

    pub fn next_char(&mut self) {
        if self.lines.len() > 0 {
            let line = &self.lines[self.cursor_position.line];

            if self.cursor_position.char < line.len() {
                self.cursor_position.char += 1;
            }
        }
    }

    pub fn prev_char(&mut self) {
        if self.lines.len() > 0 {
            if self.cursor_position.char > 0 {
                self.cursor_position.char -= 1;
            }
        }
    }

    pub fn next_line(&mut self) {
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
    }

    pub fn prev_line(&mut self) {
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
    }

    pub fn save(&mut self) {
        self.file_saved = true;
    }

    pub fn edit_mode(&mut self) {
        self.mode = Mode::Edit;
    }

    pub fn go_back(&mut self, _: KeyCode) -> bool {
        if self.mode == Mode::View {
            false
        } else {
            self.mode = Mode::View;
            true
        }
    }

    pub fn insert(&mut self, key_code: KeyCode) {
        self.file_saved = false;
        let line: &String = &self.lines[self.cursor_position.line];
        match key_code {
            KeyCode::Char(c) => {
                self.lines[self.cursor_position.line].insert(self.cursor_position.char, c);
                self.next_char();
            }
            KeyCode::Backspace if line.len() > 0 && self.cursor_position.char >= 1 => {
                let line = &mut self.lines[self.cursor_position.line];
                line.remove(self.cursor_position.char - 1);
                self.prev_char();
            }
            KeyCode::Delete if line.len() > 0 && self.cursor_position.char < line.len() => {
                let line = &mut self.lines[self.cursor_position.line];
                line.remove(self.cursor_position.char);
            }
            KeyCode::Backspace
                if self.cursor_position.line > 0 && self.cursor_position.char == 0 =>
            {
                self.prev_line();
                let line = &mut self.lines[self.cursor_position.line];
                self.cursor_position.char = line.len();

                let li = self.cursor_position.line;
                let next_li = li + 1;

                let l = self.lines[next_li].clone();
                self.lines.remove(next_li);
                self.lines[li].push_str(l.as_str());
            }
            KeyCode::Delete
                if self.cursor_position.line < self.lines.len() - 1
                    && self.cursor_position.char == line.len() =>
            {
                let li = self.cursor_position.line;
                let next_li = li + 1;

                let l = self.lines[next_li].clone();
                self.lines.remove(next_li);
                self.lines[li].push_str(l.as_str());
            }
            KeyCode::Enter => {
                let li = self.cursor_position.line;
                let ci = self.cursor_position.char;

                self.lines.insert(li + 1, String::new());
                self.next_line();

                if ci != self.lines[li].len() {
                    let p2 = String::from(&self.lines[li][ci..]);

                    self.lines[li].truncate(ci);
                    self.lines[li + 1].clear();
                    self.lines[li + 1].push_str(&p2);
                }
            }
            _ => {}
        }
    }

    fn highlight_cursor<'a>(
        &'a self,
        (line_index, line_str): (usize, &'a str),
        cp: CursorPosition,
    ) -> Line {
        let cursor_line_index = cp.line;
        let char_index = cp.char;
        if cursor_line_index == line_index && self.is_focused {
            if char_index < line_str.len() {
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
                let highlighted = " ";
                Line::from(vec![
                    Span::from(line_str),
                    Span::styled(
                        highlighted,
                        Style::default().fg(Color::Black).bg(Color::White),
                    ),
                ])
            }
        } else {
            Line::from(line_str)
        }
    }

    pub fn get_file_name(&self) -> &str {
        self.file
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap()
    }

    fn get_title(&self) -> String {
        let mut mode_str = match self.mode {
            Mode::Edit => "[Edit] ",
            Mode::View => "[View] ",
        };

        if !self.is_focused {
            mode_str = "";
        }

        let filename = self.get_file_name();
        if !self.file_saved {
            format!("{}{}*", mode_str, filename)
        } else {
            format!("{}{}", mode_str, filename)
        }
    }
}

impl Drawable for TextEditor {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        let mut block = Block::bordered().title(self.get_title());

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

fn get_insertable_key_codes() -> Vec<KeyCode> {
    let mut vec: Vec<KeyCode> = (32u8..=126u8).map(|c| KeyCode::Char(c as char)).collect();
    vec.push(KeyCode::Backspace);
    vec.push(KeyCode::Delete);
    vec.push(KeyCode::Enter);
    vec
}

fn is_insertable_key_code(key_code: KeyCode) -> bool {
    get_insertable_key_codes().contains(&key_code)
}

impl InputHandler for TextEditor {
    fn handle_input(&mut self, key_code: KeyCode) -> bool {
        match self.mode {
            Mode::Edit if is_insertable_key_code(key_code) => {
                self.insert(key_code);
                true
            }
            Mode::View | Mode::Edit => self.handle_command(key_code),
        }
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
                func: as_command!(TextEditor, next_char),
            },
            Command {
                id: "text_editor.prev_char",
                name: "Prev char",
                func: as_command!(TextEditor, prev_char),
            },
            Command {
                id: "text_editor.next_line",
                name: "Next line",
                func: as_command!(TextEditor, next_line),
            },
            Command {
                id: "text_editor.prev_line",
                name: "Prev line",
                func: as_command!(TextEditor, prev_line),
            },
            Command {
                id: "text_editor.save",
                name: "Save",
                func: as_command!(TextEditor, save),
            },
            Command {
                id: "text_editor.insert_mode",
                name: "Edit",
                func: as_command!(TextEditor, edit_mode),
            },
            Command {
                id: "text_editor.go_back",
                name: "Go back",
                func: TextEditor::go_back,
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
        self.cursor_position = CursorPosition::new();
        self.file_saved = true;
    }
}
