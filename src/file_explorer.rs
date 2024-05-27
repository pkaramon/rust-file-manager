use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};
use std::fs;
use std::path::PathBuf;

pub struct FileExplorer {
    pub current_dir: PathBuf,
    pub selected_index: usize,
    pub entries: Vec<PathBuf>,
    pub list_state: ListState,
    pub is_active: bool,
}

impl FileExplorer {
    pub fn new() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let entries = read_dir_entries(&current_dir);
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            current_dir,
            selected_index: 0,
            entries,
            list_state,
            is_active: true,
        }
    }

    pub fn change_directory(&mut self, new_dir: PathBuf) {
        self.current_dir = new_dir;
        self.entries = read_dir_entries(&self.current_dir);
        self.selected_index = 0;
        self.list_state.select(Some(self.selected_index));
    }

    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn select_next(&mut self) {
        if self.selected_index < self.entries.len() - 1 {
            self.selected_index += 1;
            self.list_state.select(Some(self.selected_index));
        }
    }

    pub fn go_back(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.change_directory(parent.to_path_buf());
        }
    }

    pub fn get_selected_file(&self) -> &PathBuf {
        &self.entries[self.selected_index]
    }

    pub fn draw<'a>(&'a mut self, f: &mut Frame, area: Rect) {
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

        if self.is_active {
            block = block.border_style(Color::Blue);
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }
}

fn read_dir_entries(dir: &PathBuf) -> Vec<PathBuf> {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|res| res.unwrap().path())
        .collect();
    entries.sort();
    entries
}
