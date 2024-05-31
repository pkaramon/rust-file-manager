use std::{
    fs::{self},
    path::PathBuf,
};

use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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
    pub modal_open: bool,
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
            modal_open: false,
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
        let _ = fs::write(self.file.clone(), self.get_text());
    }

    pub fn edit_mode(&mut self) {
        self.mode = Mode::Edit;
    }

    pub fn go_back(&mut self, _: KeyCode) -> Result<bool> {
        if self.mode == Mode::View {
            if self.file_saved {
                Ok(false)
            } else {
                self.modal_open = true;
                Ok(true)
            }
        } else {
            self.mode = Mode::View;
            Ok(true)
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

    fn get_text(&self) -> String {
        self.lines.join("\n")
    }

    fn draw_modal(&self, f: &mut Frame, area: Rect) {
        let tmp = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(area);

        let popup_wrapper = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Percentage(50),
                Constraint::Percentage(25),
            ])
            .split(tmp[1])[1];

        let v_segments = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
                Constraint::Percentage(30),
            ])
            .split(popup_wrapper);

        let question_wrapper = v_segments[1];

        let spacer_block = Block::new().borders(Borders::LEFT | Borders::RIGHT);
        let spacer = v_segments[2];

        let answer_segments = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(v_segments[3]);

        let yes_wrapper = answer_segments[0];
        let no_wrapper = answer_segments[1];

        let question_block = Block::new().borders(Borders::LEFT | Borders::RIGHT | Borders::TOP);
        let question = Paragraph::new("Save?").centered().block(question_block);

        let yes_block = Block::new().borders(Borders::LEFT | Borders::BOTTOM);
        let yes = Paragraph::new("Yes [y]").centered().block(yes_block);

        let no_block = Block::new().borders(Borders::RIGHT | Borders::BOTTOM);
        let no = Paragraph::new("No [n]").centered().block(no_block);

        f.render_widget(question, question_wrapper);
        f.render_widget(yes, yes_wrapper);
        f.render_widget(no, no_wrapper);
        f.render_widget(spacer_block, spacer);
    }
}

impl Drawable for TextEditor {
    fn draw(&self, f: &mut Frame, area: Rect) {
        if self.modal_open {
            self.draw_modal(f, area);
        } else {
            let mut block = Block::bordered().title(self.get_title());

            if self.is_focused {
                block = block.border_style(Color::Blue);
            }

            let lines: Vec<Line> = self
                .lines
                .iter()
                .enumerate()
                .map(|(index, line_str)| {
                    self.highlight_cursor((index, line_str), self.cursor_position)
                })
                .collect();

            let p = Paragraph::new(lines)
                .block(block)
                .style(Style::new().white().on_black());

            f.render_widget(p, area);
        }
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
    fn handle_input(&mut self, key_code: KeyCode) -> Result<bool> {
        if self.modal_open {
            Ok(false)
        } else {
            match self.mode {
                Mode::Edit if is_insertable_key_code(key_code) => {
                    self.insert(key_code);
                    Ok(true)
                }
                Mode::View | Mode::Edit => self.handle_command(key_code),
            }
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
    fn set_path(&mut self, path: PathBuf) -> Result<()> {
        self.file = path;

        let text = fs::read_to_string(&self.file).context("Binary file")?;
        self.lines = text.split("\n").map(|str| String::from(str)).collect();
        let text = fs::read_to_string(&self.file).context("Unable to read file")?;
        self.lines = text.split("\n").map(|str| String::from(str)).collect();
        self.cursor_position = CursorPosition::new();
        self.file_saved = true;

        Ok(())
    }

    fn confirm_modal(&mut self) {
        self.modal_open = false;
        self.save();
    }

    fn refuse_modal(&mut self) {
        self.modal_open = false;
        let _ = self.set_path(self.file.clone());
    }
}
