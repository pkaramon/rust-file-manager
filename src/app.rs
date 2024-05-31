use std::fs::OpenOptions;
use std::io::Write;

use crate::command::{Command, CommandHandler, InputHandler};
use crate::editor::{EditorEnum, NullEdtior};
use crate::file_explorer::FileExplorer;
use crate::legend::Legend;
use crate::text_editor::TextEditor;
use crate::window::{Drawable, Focusable};
use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct App {
    pub explorer: FileExplorer,
    editors: [EditorEnum; 3],
    info_message: Option<String>,
    pub legend: Legend,
    pub should_stop: bool,
}

fn log(text: &str) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log.txt")
        .context("failed to open log file")?;
    file.write_all(format!("{}\n", text).as_bytes())
        .context("failed to write to log file")?;
    Ok(())
}

impl App {
    pub fn new() -> Result<App> {
        let explorer = FileExplorer::new("explorer", true)?;

        let editors = [
            EditorEnum::PreviewExplorer(FileExplorer::new("preview_explorer", false)?),
            EditorEnum::TextEditor(TextEditor::new()),
            EditorEnum::NullEdtior(NullEdtior {}),
        ];

        let mut app = App {
            explorer,
            editors,
            legend: Legend::new(),
            should_stop: false,
            info_message: None,
        };

        log("app started")?;

        app.explorer.focus();
        app.on_selected_file_change();
        app.on_window_change();
        Ok(app)
    }

    pub fn draw(&self, f: &mut Frame) {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Min(3)])
            .split(f.size());

        let top_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main_layout[0]);

        self.explorer.draw(f, top_layout[0]);

        if let Some(error_message) = &self.info_message {
            let paragraph = Paragraph::new(error_message.clone())
                .block(Block::default().borders(Borders::ALL).title("Info"));
            f.render_widget(paragraph, top_layout[1]);
        } else {
            self.draw_editor(f, top_layout[1])
        }

        self.legend.draw(f, main_layout[1]);
    }

    pub fn on_selected_file_change(&mut self) {
        let file_option = self.explorer.get_selected_file();

        if let Some(selected_file) = file_option {
            if let Err(x) = self.provide_editor().set_path(selected_file) {
                self.info_message = Some(x.to_string());
            } else {
                self.info_message = None;
            }
        }
    }

    fn on_window_change(&mut self) {
        let commands_data: Vec<(&str, &str)> = if self.provide_editor().is_focused() {
            self.provide_editor().get_commands_data()
        } else {
            self.explorer
                .get_commands()
                .iter()
                .map(|c| (c.id, c.name))
                .collect()
        };

        self.legend.update_command_bindings(commands_data);
    }

    fn quit(&mut self, _: KeyCode) -> bool {
        self.should_stop = true;
        true
    }

    fn open_selected_file(&mut self, _: KeyCode) -> bool {
        let file_option = self.explorer.get_selected_file();
        if let Some(selected_path) = file_option {
            if !selected_path.is_dir() && self.info_message.is_none() {
                self.explorer.unfocus();
                self.provide_editor().focus();
            }
        }
        true
    }

    fn go_back(&mut self, _: KeyCode) -> bool {
        self.provide_editor().unfocus();
        self.explorer.focus();
        true
    }

    fn provide_editor(&mut self) -> &mut EditorEnum {
        let editor = if let Some(path) = self.explorer.get_selected_file() {
            if path.is_dir() {
                &mut self.editors[0]
            } else {
                &mut self.editors[1]
            }
        } else {
            &mut self.editors[2]
        };
        editor
    }

    fn draw_editor(&self, f: &mut Frame, area: Rect) {
        if let Some(path) = self.explorer.get_selected_file() {
            if path.is_dir() {
                self.editors[0].draw(f, area);
            } else {
                self.editors[1].draw(f, area);
            }
        } else {
            self.editors[2].draw(f, area);
        }
    }
}

impl InputHandler for App {
    fn handle_input(&mut self, key_code: KeyCode) -> bool {
        let mut captured = false;
        let editor = self.provide_editor();

        if editor.is_focused() {
            if editor.modal_open() {
                if key_code == KeyCode::Char('y') {
                    editor.confirm_modal();
                } else if key_code == KeyCode::Char('n') {
                    editor.refuse_modal();
                }
                captured = self.go_back(key_code);
            }
            captured |= self.provide_editor().handle_input(key_code);
        } else if self.explorer.is_focused() {
            captured |= self.explorer.handle_input(key_code);
            if captured {
                self.on_selected_file_change();
            }
        }
        if !captured {
            captured |= self.handle_command(key_code);
            if captured {
                self.on_window_change();
            }
        }
        captured
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
