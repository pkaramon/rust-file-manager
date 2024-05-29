use crossterm::event::KeyCode;
use ratatui::{
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Paragraph},
    Frame,
};

use crate::{
    binding::{get_bindings, Binding},
    command::{Command, CommandHandler},
    window::Drawable,
};

pub struct Legend {
    command_bindings_string: String,
}

struct CommandBinding<'a, T> {
    command: &'a Command<T>,
    binding: &'a Binding,
}

impl Legend {
    pub fn new() -> Self {
        Legend {
            command_bindings_string: String::new(),
        }
    }

    pub fn update_command_bindings<T: CommandHandler>(&mut self, handler: &T) {
        let bindings = get_bindings();
        let commands = handler.get_commands();

        let command_bindings: Vec<CommandBinding<T>> = commands
            .iter()
            .map(|command| {
                let binding = bindings
                    .iter()
                    .find(|binding| binding.command_id == command.id)
                    .unwrap();
                CommandBinding { command, binding }
            })
            .collect();

        let string_vec: Vec<String> = command_bindings
            .iter()
            .map(|cb| {
                let key_str = &keycode_to_string(cb.binding.key_code);
                let command_str = cb.command.name.to_string();

                format!("[{key_str}] {command_str}")
            })
            .collect();

        self.command_bindings_string = string_vec.join("  ");
    }
}

impl Drawable for Legend {
    fn draw(&mut self, f: &mut Frame, area: Rect) {
        let line = Line::from(self.command_bindings_string.clone());
        let p = Paragraph::new(line)
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
