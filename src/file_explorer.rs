use anyhow::{Context, Result};
use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::fs;
use std::path::PathBuf;

use crate::{
    command::{Command, CommandHandler, InputHandler},
    editor::Editor,
    window::{Drawable, Focusable},
};

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
    pub list_state: ListState,
    is_focused: bool,
    interactive: bool,
    name: &'static str,
}

impl FileExplorer {
    pub fn new(name: &'static str, interactive: bool) -> Result<Self> {
        let current_dir = std::env::current_dir().unwrap();
        let entries = read_dir_entries(&current_dir)?;
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Ok(Self {
            current_dir,
            selected_index: 0,
            entries,
            list_state,
            is_focused: false,
            interactive,
            name,
        })
    }

    pub fn select_previous(&mut self, _: KeyCode) -> Result<bool> {
        if !self.entries.is_empty() && self.selected_index > 0 {
            self.selected_index -= 1;
            self.list_state.select(Some(self.selected_index));
        }
        Ok(true)
    }

    pub fn select_next(&mut self, _: KeyCode) -> Result<bool> {
        if !self.entries.is_empty() && self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.list_state.select(Some(self.selected_index));
        }
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
}

impl Drawable for FileExplorer {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
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

        f.render_stateful_widget(list, area, &mut self.list_state);
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
            ]
        }
    }
}

impl InputHandler for FileExplorer {
    fn handle_input(&mut self, key_code: KeyCode) -> Result<bool> {
        self.handle_command(key_code)
    }
}

impl Editor for FileExplorer {
    fn set_path(&mut self, new_dir: PathBuf) -> Result<()> {
        self.entries = read_dir_entries(&new_dir)?;
        self.current_dir = new_dir;
        self.selected_index = 0;
        self.list_state.select(Some(self.selected_index));
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
