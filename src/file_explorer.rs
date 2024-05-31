use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::path::PathBuf;
use std::{cell::RefCell, fs};

use crate::{
    command::{Command, CommandHandler, InputHandler},
    editor::Editor,
    explorer_modal::{Modal, ModalStatus, ModalVariant},
    window::{Drawable, Focusable},
};

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
    pub list_state: RefCell<ListState>,
    is_focused: bool,
    interactive: bool,
    modal: Modal,
    task: ExplorerModalTask,
    name_filter: String,
    name: &'static str,
    sort_entries: Vec<SortEntry>,
    current_sort: usize,
}

enum ExplorerModalTask {
    DeleteFile(PathBuf),
    MoveFile(PathBuf),
    Filter,
    Noop,
}

struct SortEntry {
    name: String,
    func: fn(&mut Vec<PathBuf>) -> Result<bool>,
}

fn sort_by_name(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort();
    Ok(true)
}

fn sort_by_size(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort_by(|a, b| {
        let a_size = fs::metadata(a).unwrap().len();
        let b_size = fs::metadata(b).unwrap().len();
        b_size.cmp(&a_size)
    });
    Ok(true)
}

fn sort_by_modified_date(entries: &mut Vec<PathBuf>) -> Result<bool> {
    entries.sort_by(|a, b| {
        let a_time = fs::metadata(a).unwrap().modified().unwrap();
        let b_time = fs::metadata(b).unwrap().modified().unwrap();
        b_time.cmp(&a_time)
    });
    Ok(true)
}

impl FileExplorer {
    pub fn new(name: &'static str, interactive: bool) -> Result<Self> {
        let current_dir = std::env::current_dir().unwrap();
        let entries = read_dir_entries(&current_dir)?;
        let list_state = RefCell::new(ListState::default());
        list_state.borrow_mut().select(Some(0));
        Ok(Self {
            current_dir,
            selected_index: 0,
            entries,
            list_state,
            is_focused: false,
            interactive,
            task: ExplorerModalTask::Noop,
            modal: Modal::new(),
            name_filter: String::new(),
            sort_entries: vec![
                SortEntry {
                    name: String::from("Name"),
                    func: sort_by_name,
                },
                SortEntry {
                    name: String::from("Size"),
                    func: sort_by_size,
                },
                SortEntry {
                    name: String::from("Modified"),
                    func: sort_by_modified_date,
                },
            ],
            current_sort: 0,
            name,
        })
    }

    pub fn select_previous(&mut self, _: KeyCode) -> Result<bool> {
        if !self.entries.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.list_state
                .borrow_mut()
                .select(Some(self.selected_index));
        }
        Ok(true)
    }

    pub fn select_next(&mut self, _: KeyCode) -> Result<bool> {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.list_state
                .borrow_mut()
                .select(Some(self.selected_index));
        }
        Ok(true)
    }

    pub fn delete_current_file(&mut self, _: KeyCode) -> Result<bool> {
        if let Ok(selected_file) = self.get_selected_file() {
            self.modal.open(
                format!("Delete file: {}?", selected_file.to_str().unwrap()),
                ModalVariant::Confirmation,
            );
            self.task = ExplorerModalTask::DeleteFile(selected_file);
        } else {
            self.modal
                .open("Selected file is invalid".to_string(), ModalVariant::Error)
        }
        Ok(true)
    }

    pub fn move_current_file(&mut self, _: KeyCode) -> Result<bool> {
        if let Ok(selected_file) = self.get_selected_file() {
            self.modal.open(
                format!("Move file: {} to?", selected_file.to_str().unwrap()),
                ModalVariant::Question(selected_file.to_str().unwrap().to_string()),
            );
            self.task = ExplorerModalTask::MoveFile(selected_file);
        } else {
            self.modal
                .open("Selected file is invalid".to_string(), ModalVariant::Error)
        }
        Ok(true)
    }

    pub fn sort_entries(&mut self, _: KeyCode) -> Result<bool> {
        self.modal.open(
            "Sort by: ".to_string(),
            ModalVariant::Options(
                self.sort_entries
                    .iter()
                    .map(|entry| entry.name.clone())
                    .collect(),
                0,
            ),
        );

        Ok(true)
    }

    pub fn filter(&mut self, _: KeyCode) -> Result<bool> {
        self.modal.open(
            String::from("Search: "),
            ModalVariant::Question(String::new()),
        );
        self.task = ExplorerModalTask::Filter;
        Ok(true)
    }

    pub fn go_back(&mut self, _: KeyCode) -> Result<bool> {
        if let Some(parent) = self.current_dir.parent() {
            self.set_path(parent.to_path_buf())?;
        }
        Ok(true)
    }

    pub fn get_selected_file(&self) -> Result<PathBuf> {
        self.entries
            .get(self.selected_index)
            .cloned()
            .context("Could not get selected file")
    }

    pub fn open_selected_file(&mut self, _: KeyCode) -> Result<bool> {
        if let Ok(selected_file) = self.get_selected_file() {
            if selected_file.is_dir() {
                let _ = self.set_path(selected_file);
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn refresh(&mut self) -> Result<()> {
        self.entries = read_dir_entries(&self.current_dir)?
            .iter()
            .map(|entry| entry.clone())
            .filter(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap();
                name.contains(&self.name_filter)
            })
            .collect();

        (self.sort_entries[self.current_sort].func)(&mut self.entries)?;

        self.list_state.borrow_mut().select(Some(0));
        self.selected_index = 0;
        self.modal.is_open = false;
        Ok(())
    }

    fn handle_modal_tasks(&mut self) -> Result<()> {
        match self.modal.status {
            ModalStatus::Refused => self.modal.close(),
            ModalStatus::Confirmed => match self.modal.variant {
                ModalVariant::Confirmation => match &self.task {
                    ExplorerModalTask::DeleteFile(file) => {
                        let removal = || {
                            if file.is_dir() {
                                fs::remove_dir_all(file)
                            } else {
                                fs::remove_file(file)
                            }
                        };
                        if let Err(e) = removal() {
                            self.modal
                                .open(format!("Could not delete: {}", e), ModalVariant::Error)
                        } else {
                            self.refresh()?;
                        }
                    }
                    &_ => {}
                },
                ModalVariant::Info | ModalVariant::Error => self.modal.close(),
                ModalVariant::Question(ref answer) => match &self.task {
                    ExplorerModalTask::MoveFile(file) => {
                        let newpath = PathBuf::from(answer);
                        if let Err(e) = fs::rename(file, &newpath) {
                            self.modal
                                .open(format!("Could not move file: {}", e), ModalVariant::Error)
                        } else {
                            self.refresh()?;
                        }
                    }
                    ExplorerModalTask::Filter => {
                        self.name_filter = answer.clone();
                        self.refresh()?;
                    }
                    _ => {}
                },
                ModalVariant::Options(_, index) => {
                    if index < self.sort_entries.len() {
                        self.current_sort = index;
                        self.refresh()?;
                    } else {
                        self.modal
                            .open("Invalid option".to_string(), ModalVariant::Error);
                    }
                }
            },
            _ => {}
        }

        Ok(())
    }
}

impl Drawable for FileExplorer {
    fn draw(&self, f: &mut Frame, area: Rect) {
        if self.modal.is_open {
            self.modal.draw(f, area);
            return;
        }

        let items: Vec<ListItem> = self
            .entries
            .iter()
            .map(|entry| {
                let name = entry.file_name().unwrap().to_str().unwrap();
                ListItem::new(Span::from(name))
            })
            .collect();

        let mut block = Block::default()
            .borders(Borders::ALL)
            .title(self.current_dir.to_str().unwrap());

        if self.is_focused {
            block = block.border_style(Color::Blue);
        }

        let mut list = List::new(items).block(block);

        if self.is_focused {
            list = list
                .highlight_style(Style::default().bg(Color::Blue))
                .highlight_symbol(">> ");
        }

        let mut list_state = self.list_state.borrow_mut();
        f.render_stateful_widget(list, area, &mut *list_state);
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
    fn handle_input(&mut self, key_code: KeyCode) -> Result<bool> {
        if self.modal.is_open {
            self.modal.handle_input(key_code);
            self.handle_modal_tasks()?;
            return Ok(true);
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
        self.list_state
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
                    func: FileExplorer::delete_current_file,
                },
                Command {
                    id: "explorer.move_current_file",
                    name: "Move file",
                    func: FileExplorer::move_current_file,
                },
                Command {
                    id: "explorer.sort_entries",
                    name: "Sort",
                    func: FileExplorer::sort_entries,
                },
                Command {
                    id: "explorer.filter",
                    name: "Filter",
                    func: FileExplorer::filter,
                },
            ]
        }
    }
}
