use std::{cell::RefCell, time::SystemTime};

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
    window::Drawable,
};

#[derive(Clone)]
struct AnimationData {
    time: SystemTime,
    dir: bool,
    scroll_pos: u16,
}

pub struct Legend {
    command_bindings_string: String,
    anim: RefCell<AnimationData>,
}

struct CommandBinding<'a> {
    command: &'a (&'static str, &'static str),
    binding: &'a Binding,
}

impl Legend {
    pub fn new() -> Self {
        Legend {
            command_bindings_string: String::new(),
            anim: RefCell::new(AnimationData {
                time: SystemTime::now(),
                dir: false,
                scroll_pos: 0,
            }),
        }
    }

    pub fn update_command_bindings(&mut self, commands: Vec<(&'static str, &'static str)>) {
        let bindings = get_bindings();

        let command_bindings: Vec<CommandBinding> = commands
            .iter()
            .map(|command| {
                let binding = bindings
                    .iter()
                    .find(|binding| binding.command_id == command.0)
                    .unwrap();
                CommandBinding { command, binding }
            })
            .collect();

        let string_vec: Vec<String> = command_bindings
            .iter()
            .map(|cb| {
                let key_str = &keycode_to_string(cb.binding.key_code);
                let command_str = cb.command.1.to_string();

                format!("[{key_str}] {command_str}")
            })
            .collect();

        self.command_bindings_string = string_vec.join("  ");
        self.command_bindings_string.insert(0, ' ');
        self.command_bindings_string.push(' ');
    }

    fn calc_anim_data(&self, area: Rect) {
        let x_margin = 2u16;

        let legend_width = self.command_bindings_string.len() as u16 + x_margin;

        if legend_width <= area.width {
            self.anim.replace(AnimationData {
                time: SystemTime::now(),
                dir: false,
                scroll_pos: 0,
            });
            return;
        }
        let anim = self.anim.clone().into_inner();
        let now = SystemTime::now();
        let scroll_pos = anim.scroll_pos;

        if let Ok(diff) = now.duration_since(anim.time) {
            if diff.as_millis() < 250 {
                return;
            }
        } else {
            return;
        }

        let (new_dir, new_scroll_pos) = if !anim.dir {
            if scroll_pos + area.width < legend_width {
                (false, scroll_pos + 1)
            } else {
                (true, scroll_pos - 1)
            }
        } else {
            if scroll_pos == 0 {
                (false, scroll_pos + 1)
            } else {
                (true, scroll_pos - 1)
            }
        };

        self.anim.replace(AnimationData {
            time: SystemTime::now(),
            dir: new_dir,
            scroll_pos: new_scroll_pos,
        });
    }
}

impl Drawable for Legend {
    fn draw(&self, f: &mut Frame, area: Rect) {
        let line = Line::from(self.command_bindings_string.clone());

        self.calc_anim_data(area);

        let p = Paragraph::new(line)
            .block(Block::bordered())
            .style(Style::new().white().on_black())
            .scroll((0, self.anim.borrow().scroll_pos));

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
