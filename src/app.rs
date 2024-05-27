use std::fs;

use crate::file_explorer::FileExplorer;
use crate::legend::Legend;
use crate::navigation::handle_input;
use crate::text_editor::TextEditor;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

#[derive(PartialEq)]
pub enum ActiveWindow {
    Explorer,
    Editor,
}

pub struct App {
    pub explorer: FileExplorer,
    pub preview_explorer: FileExplorer,
    pub text_editor: TextEditor,
    pub active_window: ActiveWindow,
    pub should_stop: bool,
}

impl App {
    pub fn new() -> App {
        App {
            explorer: FileExplorer::new(),
            preview_explorer: FileExplorer::new(),
            text_editor: TextEditor::new(),
            active_window: ActiveWindow::Explorer,
            should_stop: false,
        }
    }

    pub fn open_selected_file(&mut self) {
        let selected_path = self.explorer.get_selected_file();
        if selected_path.is_dir() {
            self.explorer.change_directory(selected_path.clone());
        } else {
            self.activate_editor();
        }
    }

    pub fn draw(&mut self, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[0]);

        self.explorer.draw(f, top_layout[0]);

        if self.explorer.get_selected_file().is_dir() {
            self.preview_explorer.draw(f, top_layout[1]);
        } else {
            self.text_editor.draw(f, top_layout[1]);
        }

        let mut l = Legend {};
        l.draw(f, main_layout[1]);
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        handle_input(self, key_code);
    }

    pub fn activate_explorer(&mut self) {
        self.preview_explorer.is_active = false;
        self.text_editor.is_active = false;
        self.active_window = ActiveWindow::Explorer;
        self.explorer.is_active = true;
    }

    pub fn activate_editor(&mut self) {
        self.preview_explorer.is_active = false;
        self.text_editor.is_active = false;
        self.explorer.is_active = false;
        self.active_window = ActiveWindow::Editor;
        let selected_file = self.explorer.get_selected_file();
        if selected_file.is_dir() {
            self.preview_explorer
                .change_directory(selected_file.clone());
            self.preview_explorer.is_active = true;
        } else {
            let content = fs::read_to_string(selected_file.clone()).expect("Unable to read file");
            self.text_editor.set_text(content);
            self.text_editor.is_active = true;
        }
    }
}
