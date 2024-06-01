use crossterm::event::KeyCode;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    Frame,
};

use crate::window::Drawable;

pub trait ModalVariant {
    fn handle_input(&mut self, state: &mut ModalState, key_code: KeyCode);
    fn draw(&self, f: &mut Frame, area: Rect);
}

pub struct Modal {
    pub state: ModalState,
    pub variant_trait: Box<dyn ModalVariant>,
}

pub struct ModalState {
    pub is_open: bool,
}

impl Modal {
    pub fn new(variant: Box<dyn ModalVariant>) -> Modal {
        Self {
            state: ModalState { is_open: true },
            variant_trait: variant,
        }
    }

    pub fn is_open(&self) -> bool {
        self.state.is_open
    }

    pub fn close(&mut self) {
        self.state.is_open = false;
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        self.variant_trait.handle_input(&mut self.state, key_code);
    }
}

impl Drawable for Modal {
    fn draw(&self, f: &mut Frame, area: Rect) {
        if !self.state.is_open {
            return;
        }

        let tmp = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(area);

        let popup_wrapper = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(tmp[1])[1];

        self.variant_trait.draw(f, popup_wrapper);
    }
}
