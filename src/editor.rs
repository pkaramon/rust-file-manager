use std::path::PathBuf;

use crossterm::event::KeyCode;
use ratatui::{layout::Rect, Frame};

use crate::{
    command::{CommandHandler, InputHandler},
    file_explorer::FileExplorer,
    text_editor::TextEditor,
    window::{Drawable, Focusable},
};

pub enum EditorEnum {
    TextEditor(TextEditor),
    PreviewExplorer(FileExplorer),
}

pub trait Editor: Drawable + Focusable + InputHandler {
    fn set_path(&mut self, path: PathBuf);
    fn confirm_modal(&mut self) {}
    fn refuse_modal(&mut self) {}
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

    pub fn handle_input(&mut self, key_code: KeyCode) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.handle_input(key_code),
            EditorEnum::PreviewExplorer(editor) => editor.handle_input(key_code),
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

    pub fn modal_open(&self) -> bool {
        match self {
            EditorEnum::TextEditor(editor) => editor.modal_open,
            _ => false,
        }
    }

    pub fn confirm_modal(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.confirm_modal(),
            EditorEnum::PreviewExplorer(editor) => editor.confirm_modal(),
        }
    }

    pub fn refuse_modal(&mut self) {
        match self {
            EditorEnum::TextEditor(editor) => editor.refuse_modal(),
            EditorEnum::PreviewExplorer(editor) => editor.refuse_modal(),
        }
    }
}
