use anyhow::{Context, Result};
use byte_unit::Byte;
use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Row, Table, TableState},
    Frame,
};
use std::path::PathBuf;
use std::{
    cell::RefCell,
    fs,
    sync::mpsc::{channel, Receiver, Sender},
};

use crate::{
    command::{Command, CommandHandler, InputHandler},
    editor::Editor,
    modal::Modal,
    modal_variants::{ConfirmationVariant, InfoVariant, OptionsVariant, QuestionVariant},
    sort_entries::SORT_ENTRIES,
    window::{Drawable, Focusable},
};

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
    pub table_state: RefCell<TableState>,
    interactive: bool,
    name: &'static str,

    modal: Modal,
    name_filter: String,
    current_sort: usize,
    is_focused: bool,

    sender: Sender<ExplorerTask>,
    receiver: Receiver<ExplorerTask>,
}

pub enum ExplorerTask {
    DeleteFile(PathBuf),
    MoveFile(PathBuf, String),
    CreateFile(String),
    Sort(usize),
    Filter(String),
}

impl FileExplorer {
    pub fn new(name: &'static str, interactive: bool) -> Result<Self> {
        let current_dir = std::env::current_dir().unwrap();
        let entries = read_dir_entries(&current_dir)?;
        let list_state = RefCell::new(TableState::default());
        list_state.borrow_mut().select(Some(0));

        let (sender, receiver) = channel();

        let mut modal = Modal::new(Box::new(InfoVariant::new(String::new())));
        modal.close();
        Ok(Self {
            current_dir,
            selected_index: 0,
            entries,
            table_state: list_state,
            is_focused: false,
            interactive,
            name_filter: String::new(),
            modal,
            sender,
            receiver,
            current_sort: 0,
            name,
        })
    }

    pub fn select_previous(&mut self, _: KeyCode) -> bool {
        if !self.entries.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.table_state
                .borrow_mut()
                .select(Some(self.selected_index));
        }
        true
    }

    pub fn select_next(&mut self, _: KeyCode) -> bool {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.table_state
                .borrow_mut()
                .select(Some(self.selected_index));
        }
        true
    }

    pub fn prompt_for_delete_current_file(&mut self, _: KeyCode) -> bool {
        if let Some(selected_file) = self.get_selected_file() {
            let sender = self.sender.clone();
            self.modal = Modal::new(Box::new(ConfirmationVariant::new(
                format!("Delete file: {}?", selected_file.to_str().unwrap()),
                Box::new(move |_| {
                    sender
                        .send(ExplorerTask::DeleteFile(selected_file.clone()))
                        .unwrap();
                }),
            )));
        } else {
            self.open_info_modal("Selected file is invalid".to_string());
        }
        true
    }

    pub fn prompt_for_move_file(&mut self, _: KeyCode) -> bool {
        if let Some(selected_file) = self.get_selected_file() {
            let sender = self.sender.clone();
            self.modal = Modal::new(Box::new(QuestionVariant::new(
                format!("Move file: {} to?", selected_file.to_str().unwrap()),
                String::from(selected_file.to_str().unwrap()),
                Box::new(move |answer| {
                    sender
                        .send(ExplorerTask::MoveFile(selected_file.clone(), answer))
                        .unwrap();
                }),
            )));
        } else {
            self.open_info_modal("Selected file is invalid".to_string());
        }
        true
    }

    pub fn prompt_for_sorting_criterion(&mut self, _: KeyCode) -> bool {
        let sender = self.sender.clone();
        self.modal = Modal::new(Box::new(OptionsVariant::new(
            "Sort by: ".to_string(),
            SORT_ENTRIES
                .iter()
                .map(|entry| entry.name.to_string())
                .collect(),
            Box::new(move |index| {
                sender.send(ExplorerTask::Sort(index)).unwrap();
            }),
        )));

        true
    }

    pub fn prompt_for_new_file(&mut self, _: KeyCode) -> bool {
        let sender = self.sender.clone();
        self.modal = Modal::new(Box::new(QuestionVariant::new(
            String::from("Create file:"),
            String::new(),
            Box::new(move |answer| {
                sender.send(ExplorerTask::CreateFile(answer)).unwrap();
            }),
        )));

        true
    }

    pub fn prompt_for_new_filter(&mut self, _: KeyCode) -> bool {
        let sender = self.sender.clone();
        self.modal = Modal::new(Box::new(QuestionVariant::new(
            String::from("Filter: "),
            String::new(),
            Box::new(move |answer| {
                sender.send(ExplorerTask::Filter(answer)).unwrap();
            }),
        )));

        true
    }

    pub fn go_back(&mut self, _: KeyCode) -> bool {
        if let Some(parent) = self.current_dir.parent() {
            let _ = self.set_path(parent.to_path_buf());
        }
        true
    }

    fn open_info_modal(&mut self, message: String) {
        self.modal = Modal::new(Box::new(InfoVariant::new(message)));
    }

    pub fn get_selected_file(&self) -> Option<PathBuf> {
        self.entries.get(self.selected_index).cloned()
    }

    pub fn open_selected_file(&mut self, _: KeyCode) -> bool {
        if let Some(selected_file) = self.get_selected_file() {
            if selected_file.is_dir() {
                let _ = self.set_path(selected_file);
                return true;
            }
        }
        false
    }

    fn refresh(&mut self) -> Result<()> {
        self.entries = read_dir_entries(&self.current_dir)?
            .iter()
            .map(|entry| entry.clone())
            .filter(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap();
                name.to_lowercase()
                    .contains(&self.name_filter.to_lowercase())
            })
            .collect();

        (SORT_ENTRIES[self.current_sort].func)(&mut self.entries)?;
        self.table_state.borrow_mut().select(Some(0));
        self.selected_index = 0;
        Ok(())
    }

    fn dispatch_on_task(&mut self, task: ExplorerTask) -> Result<()> {
        Ok(match task {
            ExplorerTask::CreateFile(name) => {
                let new_file = self.current_dir.join(&name);
                if new_file.try_exists().unwrap_or(false) {
                    self.open_info_modal("File already exists".to_string());
                } else {
                    let create = || -> Result<()> {
                        if name.ends_with("/") {
                            Ok(fs::create_dir(new_file)?)
                        } else {
                            fs::File::create(&new_file)?;
                            Ok(())
                        }
                    };
                    match create() {
                        Ok(_) => {}
                        Err(_) => self.open_info_modal("Could not create the file".to_string()),
                    }
                }
                self.refresh()?;
            }
            ExplorerTask::DeleteFile(filepath) => {
                let removal = || {
                    if filepath.is_dir() {
                        fs::remove_dir_all(filepath)
                    } else {
                        fs::remove_file(filepath)
                    }
                };

                if let Err(e) = removal() {
                    self.open_info_modal(format!("Could not delete: {}", e));
                } else {
                    self.refresh()?;
                }
            }
            ExplorerTask::MoveFile(original, new_path) => {
                let newpath = PathBuf::from(new_path);
                if let Err(e) = fs::rename(original, &newpath) {
                    self.open_info_modal(format!("Could not move file: {}", e));
                } else {
                    self.refresh()?;
                }
            }
            ExplorerTask::Sort(entry_index) => {
                self.current_sort = entry_index;
                self.refresh()?;
            }
            ExplorerTask::Filter(search) => {
                self.name_filter = search;
                self.refresh()?;
            }
        })
    }
}

impl Drawable for FileExplorer {
    fn draw(&self, f: &mut Frame, area: Rect) {
        if self.modal.is_open() {
            self.modal.draw(f, area);
            return;
        }

        let file_rows: Vec<Row> = self
            .entries
            .iter()
            .map(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap();
                let file_type = if entry.is_dir() { "dir" } else { "file" };
                if let Ok(file_metadata) = entry.metadata() {
                    let file_size = file_metadata.len();
                    let readable_size =
                        Byte::from_u64(file_size).get_appropriate_unit(byte_unit::UnitType::Binary);

                    Row::new([
                        Span::from(file_type).style(Style::default().fg(Color::Green)),
                        Span::from(format!("{readable_size:.2}")),
                        Span::from(name),
                    ])
                } else {
                    Row::new([
                        Span::from(file_type).style(Style::default().fg(Color::Green)),
                        Span::from("?"),
                        Span::from(name),
                    ])
                }
            })
            .collect();

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.current_dir.to_str().unwrap());

        if self.is_focused {
            block = block.border_style(Color::Blue);
        }

        let mut table_state = self.table_state.borrow_mut();
        let widths = [
            Constraint::Percentage(10),
            Constraint::Percentage(20),
            Constraint::Percentage(70),
        ];
        let mut table = Table::new(file_rows, widths)
            .block(block)
            .header(Row::new(vec!["Type", "Size", "Name"]));

        if self.is_focused {
            table = table
                .highlight_symbol(">>")
                .highlight_style(Style::default().bg(Color::Blue));
        }

        f.render_stateful_widget(table, area, &mut table_state);
    }
}

impl Focusable for FileExplorer {
    fn focus(&mut self) {
        if self.interactive {
            self.is_focused = true;
        }
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }

    fn is_focused(&self) -> bool {
        self.is_focused && self.interactive
    }
}

impl InputHandler for FileExplorer {
    fn handle_input(&mut self, key_code: KeyCode) -> bool {
        if self.modal.is_open() {
            self.modal.handle_input(key_code);
            if let Ok(task) = self.receiver.try_recv() {
                let _ = self.dispatch_on_task(task);
            }
            true
        } else {
            self.handle_command(key_code)
        }
    }
}

impl Editor for FileExplorer {
    fn set_path(&mut self, new_dir: PathBuf) -> Result<()> {
        self.entries = read_dir_entries(&new_dir)?;
        self.current_dir = new_dir;
        self.selected_index = 0;
        self.name_filter = String::new();
        self.current_sort = 0;
        self.table_state
            .borrow_mut()
            .select(Some(self.selected_index));
        Ok(())
    }
}

fn read_dir_entries(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .context("Could not read directory entries")?
        .filter(|res| res.is_ok())
        .map(|res| res.unwrap().path())
        .collect();
    entries.sort();
    Ok(entries)
}

impl CommandHandler for FileExplorer {
    fn get_name(&self) -> &'static str {
        self.name
    }
    fn get_commands(&self) -> Vec<Command<Self>> {
        if !self.interactive {
            vec![]
        } else {
            vec![
                Command {
                    id: "explorer.select_previous_file",
                    name: "Prev file",
                    func: FileExplorer::select_previous,
                },
                Command {
                    id: "explorer.select_next_file",
                    name: "Next file",
                    func: FileExplorer::select_next,
                },
                Command {
                    id: "explorer.go_back",
                    name: "Back",
                    func: FileExplorer::go_back,
                },
                Command {
                    id: "explorer.open_selected_file",
                    name: "Open file",
                    func: FileExplorer::open_selected_file,
                },
                Command {
                    id: "explorer.delete_current_file",
                    name: "Delete file",
                    func: FileExplorer::prompt_for_delete_current_file,
                },
                Command {
                    id: "explorer.move_current_file",
                    name: "Move file",
                    func: FileExplorer::prompt_for_move_file,
                },
                Command {
                    id: "explorer.sort_entries",
                    name: "Sort",
                    func: FileExplorer::prompt_for_sorting_criterion,
                },
                Command {
                    id: "explorer.create_file",
                    name: "New file",
                    func: FileExplorer::prompt_for_new_file,
                },
                Command {
                    id: "explorer.filter",
                    name: "Filter",
                    func: FileExplorer::prompt_for_new_filter,
                },
            ]
        }
    }
}
