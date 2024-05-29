use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    binding::{get_bindings, Binding},
    command::{get_commands, Command},
};

pub struct Legend {}

struct CommandBinding<'a> {
    command: &'a Command,
    binding: &'a Binding,
}

impl Legend {
    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let bindings = get_bindings();
        let commands = get_commands();

        let command_bindings: Vec<CommandBinding> = bindings
            .iter()
            .map(|binding| {
                let command = commands
                    .iter()
                    .find(|c| c.id == binding.command_id)
                    .unwrap();
                CommandBinding { command, binding }
            })
            .collect();

        let command_binding_strings: Vec<Span> = command_bindings
            .iter()
            .map(|cb| {
                let key_str = &keycode_to_string(cb.binding.key_code);

                let command_str = cb.command.name.to_string();

                Span::raw(format!("[{key_str}] {command_str}  "))
            })
            .collect();

        let text = Line::from(command_binding_strings);
        let p = Paragraph::new(text)
            .block(Block::bordered())
            .style(Style::new().white().on_black());

        f.render_widget(p, area);
    }
}

fn keycode_to_string(keycode: KeyCode) -> String {
    match keycode {
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Left => "Left Arrow".to_string(),
        KeyCode::Right => "Right Arrow".to_string(),
        KeyCode::Up => "Up Arrow".to_string(),
        KeyCode::Down => "Down Arrow".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "Page Up".to_string(),
        KeyCode::PageDown => "Page Down".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "Back Tab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Insert => "Insert".to_string(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Esc => "Escape".to_string(),
        KeyCode::Null => "Null".to_string(),
        KeyCode::CapsLock => "Caps Lock".to_string(),
        KeyCode::ScrollLock => "Scroll Lock".to_string(),
        KeyCode::NumLock => "Num Lock".to_string(),
        KeyCode::PrintScreen => "Print Screen".to_string(),
        KeyCode::Pause => "Pause".to_string(),
        KeyCode::Menu => "Menu".to_string(),
        _ => "Unknown key".to_string(),
    }
}
