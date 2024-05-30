use std::fs::OpenOptions;
use std::io::Write;

use crate::command::{Command, CommandHandler};
use crate::editor::EditorEnum;
use crate::file_explorer::FileExplorer;
use crate::legend::Legend;
use crate::text_editor::TextEditor;
use crate::window::{Drawable, Focusable};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::Frame;

pub struct App {
    pub explorer: FileExplorer,
    editors: [EditorEnum; 2],
    pub legend: Legend,
    pub should_stop: bool,
}

fn log(text: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .unwrap();
    let _ = file.write_all(format!("{}\n", text).as_bytes());
}

impl App {
    pub fn new() -> App {
        let explorer = FileExplorer::new("explorer", true);

        let editors = [
            EditorEnum::PreviewExplorer(FileExplorer::new("preview_explorer", false)),
            EditorEnum::TextEditor(TextEditor::new()),
        ];

        let mut app = App {
            explorer,
            editors,
            legend: Legend::new(),
            should_stop: false,
        };

        log("app started");

        app.explorer.focus();
        app.on_selected_file_change();
        app.on_window_change();
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

        self.provide_editor().draw(f, top_layout[1]);

        self.legend.draw(f, main_layout[1]);
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        let mut captured = false;

        let editor = self.provide_editor();

        if editor.is_focused() {
            captured |= editor.handle(key_code);
        } else if self.explorer.is_focused() {
            captured |= self.explorer.handle(key_code);
            if captured {
                self.on_selected_file_change();
            }
        }
        if !captured {
            captured |= self.handle(key_code);
            if captured {
                self.on_window_change();
            }
        }
    }

    fn on_selected_file_change(&mut self) {
        let selected_file = self.explorer.get_selected_file();
        self.provide_editor().set_path(selected_file);
    }

    fn on_window_change(&mut self) {
        let commands_data = if self.provide_editor().is_focused() {
            self.provide_editor().get_commands_data()
        } else {
            self.explorer
                .get_commands()
                .iter()
                .map(|c| (c.id, c.name))
                .collect()
        };

        log("window changed");

        self.legend.update_command_bindings(commands_data);
    }

    fn quit(&mut self, _: KeyCode) -> bool {
        self.should_stop = true;
        true
    }

    fn open_selected_file(&mut self, _: KeyCode) -> bool {
        let selected_path = self.explorer.get_selected_file();
        if !selected_path.is_dir() {
            self.explorer.unfocus();
            self.provide_editor().focus();
        }
        true
    }

    fn go_back(&mut self, _: KeyCode) -> bool {
        self.provide_editor().unfocus();
        self.explorer.focus();
        true
    }

    fn provide_editor(&mut self) -> &mut EditorEnum {
        let path = self.explorer.get_selected_file();
        let editor = if path.is_dir() {
            &mut self.editors[0]
        } else {
            &mut self.editors[1]
        };
        editor
    }
}

impl CommandHandler for App {
    fn get_name(&self) -> &'static str {
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
