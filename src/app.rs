use std::fs::OpenOptions;
use std::io::Write;

use crate::command::{Command, CommandHandler, InputHandler};
use crate::editor::EditorEnum;
use crate::file_explorer::FileExplorer;
use crate::legend::Legend;
use crate::text_editor::TextEditor;
use crate::window::{Drawable, Focusable};
use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct App {
    pub explorer: FileExplorer,
    editors: [EditorEnum; 2],
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
        app.on_selected_file_change()?;
        app.on_window_change()?;
        Ok(app)
    }

    pub fn draw(&mut self, f: &mut Frame) -> Result<()> {
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
            self.provide_editor()?.draw(f, top_layout[1]);
        }

        self.legend.draw(f, main_layout[1]);

        Ok(())
    }

    fn on_selected_file_change(&mut self) -> Result<()> {
        if let Ok(selected_file) = self.explorer.get_selected_file() {
            if let Err(x) = self.provide_editor()?.set_path(selected_file) {
                self.info_message = Some(x.to_string());
            } else {
                self.info_message = None;
            }
        }

        Ok(())
    }

    fn on_window_change(&mut self) -> Result<()> {
        let commands_data: Vec<(&str, &str)> = if self.provide_editor()?.is_focused() {
            self.provide_editor()?.get_commands_data()
        } else {
            self.explorer
                .get_commands()
                .iter()
                .map(|c| (c.id, c.name))
                .collect()
        };

        log("window changed")?;

        self.legend.update_command_bindings(commands_data);
        Ok(())
    }

    fn quit(&mut self, _: KeyCode) -> Result<bool> {
        self.should_stop = true;
        Ok(true)
    }

    fn open_selected_file(&mut self, _: KeyCode) -> Result<bool> {
        if let Ok(selected_path) = self.explorer.get_selected_file() {
            if !selected_path.is_dir() && self.info_message.is_none() {
                self.explorer.unfocus();
                self.provide_editor()?.focus();
            }
        }
        Ok(true)
    }

    fn go_back(&mut self, _: KeyCode) -> Result<bool> {
        self.provide_editor()?.unfocus();
        self.explorer.focus();
        Ok(true)
    }

    fn provide_editor(&mut self) -> Result<&mut EditorEnum> {
        let path = self.explorer.get_selected_file()?;
        let editor = if path.is_dir() {
            &mut self.editors[0]
        } else {
            &mut self.editors[1]
        };
        Ok(editor)
    }
}

impl InputHandler for App {
    fn handle_input(&mut self, key_code: KeyCode) -> Result<bool> {
        let mut captured = false;
        let editor = self.provide_editor();

        match editor {
            Ok(editor) if editor.is_focused() => {
                if editor.modal_open() {
                    if key_code == KeyCode::Char('y') {
                        editor.confirm_modal();
                    } else if key_code == KeyCode::Char('n') {
                        editor.refuse_modal();
                    }
                    captured = self.go_back(key_code)?;
                }

                captured |= self
                    .provide_editor()
                    .map_or(false, |e| e.handle_input(key_code).unwrap_or(false));
            }
            _ => {
                if self.explorer.is_focused() {
                    captured |= self.explorer.handle_input(key_code)?;
                    if captured {
                        self.on_selected_file_change()?;
                    }
                }
            }
        }

        if !captured {
            captured |= self.handle_command(key_code)?;
            if captured {
                let _ = self.on_window_change();
            }
        }

        Ok(captured)
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
