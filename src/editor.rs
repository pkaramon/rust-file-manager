use std::path::PathBuf;

use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use crate::{
    command::CommandHandler,
    file_explorer::FileExplorer,
    text_editor::TextEditor,
    window::{Drawable, Focusable},
};

pub enum EditorEnum {
    TextEditor(TextEditor),
    PreviewExplorer(FileExplorer),
}

pub trait Editor: Drawable + Focusable + CommandHandler {
    fn set_path(&mut self, path: PathBuf);
}

impl EditorEnum {
    pub fn set_path(&mut self, path: PathBuf) {
        match self {
            EditorEnum::TextEditor(editor) => editor.set_path(path),
            EditorEnum::PreviewExplorer(editor) => editor.set_path(path),
        }
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        match self {
            EditorEnum::TextEditor(editor) => editor.draw(f, area),
            EditorEnum::PreviewExplorer(editor) => editor.draw(f, area),
        }
    }

    pub fn is_focused(&self) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.is_focused(),
            EditorEnum::PreviewExplorer(editor) => editor.is_focused(),
        }
    }

    pub fn focus(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.focus(),
            EditorEnum::PreviewExplorer(editor) => editor.focus(),
        }
    }

    pub fn unfocus(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.unfocus(),
            EditorEnum::PreviewExplorer(editor) => editor.unfocus(),
        }
    }

    pub fn handle(&mut self, key_code: KeyCode) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.handle(key_code),
            EditorEnum::PreviewExplorer(editor) => editor.handle(key_code),
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
        }
    }
}
