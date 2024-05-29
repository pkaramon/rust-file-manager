use std::fs::{self};

use crate::command::{Command, CommandHandler};
use crate::file_explorer::FileExplorer;
use crate::legend::Legend;
use crate::text_editor::TextEditor;
use crate::window::{Drawable, Focusable};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

pub struct App {
    pub explorer: FileExplorer,
    pub preview_explorer: FileExplorer,
    pub text_editor: TextEditor,
    pub legend: Legend,
    pub should_stop: bool,
}

impl App {
    pub fn new() -> App {
        let explorer = FileExplorer::new("explorer");
        let preview_explorer = FileExplorer::new("preview_explorer");

        let mut app = App {
            explorer,
            preview_explorer,
            text_editor: TextEditor::new(),
            legend: Legend::new(),
            should_stop: false,
        };

        app.explorer.focus();
        app.update_view();
        app
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

        self.legend.draw(f, main_layout[1]);
    }

    fn focused_window_handle_input(&mut self, key_code: KeyCode) {
        let mut captured = false;

        if self.text_editor.is_focused() {
            captured |= self.text_editor.handle(key_code);
        } else if self.explorer.is_focused() {
            captured |= self.explorer.handle(key_code);
        }
        if !captured {
            self.handle(key_code);
        }
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        self.focused_window_handle_input(key_code);
        self.update_view();
    }

    fn update_view(&mut self) {
        let selected_file = self.explorer.get_selected_file().clone();
        if selected_file.is_dir() {
            self.preview_explorer
                .change_directory(selected_file.clone());
        } else {
            let content = fs::read_to_string(selected_file.clone())
                .unwrap_or("Unable to read file".to_string());
            self.text_editor.set_text(content);
        }

        if self.text_editor.is_focused() {
            self.legend.update_command_bindings(&mut self.text_editor);
        } else if self.explorer.is_focused() {
            self.legend.update_command_bindings(&self.explorer);
        }
    }

    fn quit(&mut self, _: KeyCode) -> bool {
        self.should_stop = true;
        true
    }

    fn open_selected_file(&mut self, _: KeyCode) -> bool {
        let selected_path = self.explorer.get_selected_file().clone();
        if !selected_path.is_dir() {
            self.explorer.unfocus();
            self.text_editor.focus();
        }
        true
    }

    fn go_back(&mut self, _: KeyCode) -> bool {
        self.text_editor.unfocus();
        self.explorer.focus();
        true
    }
}

impl CommandHandler for App {
    fn get_name(&mut self) -> &'static str {
        "app"
    }
    fn get_commands(&self) -> Vec<Command<App>> {
        vec![
            Command {
                id: "app.quit",
                name: "Quit",
                func: App::quit,
            },
            Command {
                id: "app.go_back",
                name: "Back",
                func: App::go_back,
            },
            Command {
                id: "app.open_selected_file",
                name: "Open file",
                func: App::open_selected_file,
            },
        ]
    }
}
