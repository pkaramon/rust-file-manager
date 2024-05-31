use std::path::PathBuf;

use anyhow::{Ok, Result};
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    command::{CommandHandler, InputHandler},
    file_explorer::FileExplorer,
    text_editor::TextEditor,
    window::{Drawable, Focusable},
};

pub enum EditorEnum {
    TextEditor(TextEditor),
    PreviewExplorer(FileExplorer),
    NullEdtior(NullEdtior),
}

pub trait Editor: Drawable + Focusable + InputHandler {
    fn set_path(&mut self, path: PathBuf) -> Result<()>;
}

impl EditorEnum {
    pub fn set_path(&mut self, path: PathBuf) -> Result<()> {
        match self {
            EditorEnum::TextEditor(editor) => editor.set_path(path),
            EditorEnum::PreviewExplorer(editor) => editor.set_path(path),
            EditorEnum::NullEdtior(editor) => editor.set_path(path),
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        match self {
            EditorEnum::TextEditor(editor) => editor.draw(f, area),
            EditorEnum::PreviewExplorer(editor) => editor.draw(f, area),
            EditorEnum::NullEdtior(editor) => editor.draw(f, area),
        }
    }

    pub fn is_focused(&self) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.is_focused(),
            EditorEnum::PreviewExplorer(editor) => editor.is_focused(),
            EditorEnum::NullEdtior(editor) => editor.is_focused(),
        }
    }

    pub fn focus(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.focus(),
            EditorEnum::PreviewExplorer(editor) => editor.focus(),
            EditorEnum::NullEdtior(editor) => editor.focus(),
        }
    }

    pub fn unfocus(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.unfocus(),
            EditorEnum::PreviewExplorer(editor) => editor.unfocus(),
            EditorEnum::NullEdtior(editor) => editor.unfocus(),
        }
    }

    pub fn handle_input(&mut self, key_code: KeyCode) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.handle_input(key_code),
            EditorEnum::PreviewExplorer(editor) => editor.handle_input(key_code),
            EditorEnum::NullEdtior(editor) => editor.handle_input(key_code),
        }
    }

    pub fn get_commands_data(&self) -> Vec<(&'static str, &'static str)> {
        match self {
            EditorEnum::TextEditor(editor) => editor
                .get_commands()
                .iter()
                .map(|c| (c.id, c.name))
                .collect(),
            EditorEnum::PreviewExplorer(editor) => editor
                .get_commands()
                .iter()
                .map(|c| (c.id, c.name))
                .collect(),
            EditorEnum::NullEdtior(_) => vec![],
        }
    }

    pub fn modal_open(&self) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.modal_open,
            _ => false,
        }
    }
}

pub struct NullEdtior {
    pub message: Option<String>,
}

impl Drawable for NullEdtior {
    fn draw(&self, f: &mut Frame, area: Rect) {
        let centered = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
            ])
            .split(area)[1];
        let block = Block::bordered();

        if let Some(message) = &self.message {
            let paragraph = Paragraph::new(message.clone()).centered();
            f.render_widget(paragraph, centered);
        }
        f.render_widget(block, area);
    }
}

impl InputHandler for NullEdtior {
    fn handle_input(&mut self, _: KeyCode) -> bool {
        false
    }
}

impl Focusable for NullEdtior {
    fn focus(&mut self) {}

    fn unfocus(&mut self) {}

    fn is_focused(&self) -> bool {
        false
    }
}

impl Editor for NullEdtior {
    fn set_path(&mut self, _: PathBuf) -> Result<()> {
        Ok(())
    }
}
