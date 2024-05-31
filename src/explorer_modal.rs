use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::window::Drawable;

pub struct Modal {
    pub is_open: bool,
    pub message: String,
    pub status: ModalStatus,
    pub variant: ModalVariant,
}

pub enum ModalStatus {
    Confirmed,
    Refused,
    Waiting,
}

pub enum ModalVariant {
    Confirmation,
    Error,
    Info,
    Question(String),
}

impl Modal {
    pub fn new() -> Self {
        Self {
            is_open: false,
            message: String::new(),
            status: ModalStatus::Waiting,
            variant: ModalVariant::Info,
        }
    }

    pub fn open(&mut self, message: String, variant: ModalVariant) {
        self.is_open = true;
        self.message = message;
        self.variant = variant;
        self.status = ModalStatus::Waiting;
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.status = ModalStatus::Waiting;
    }

    pub fn handle_input(&mut self, key_code: KeyCode) {
        match self.variant {
            ModalVariant::Confirmation | ModalVariant::Error | ModalVariant::Info => match key_code
            {
                KeyCode::Char('y') => {
                    self.status = ModalStatus::Confirmed;
                }
                KeyCode::Char('n') => {
                    self.status = ModalStatus::Refused;
                }
                _ => {}
            },
            ModalVariant::Question(ref mut content) => match key_code {
                KeyCode::Backspace => {
                    content.pop();
                }
                KeyCode::Char(c) => {
                    content.push(c);
                }
                KeyCode::Enter => {
                    self.status = ModalStatus::Confirmed;
                }
                KeyCode::Esc => {
                    self.status = ModalStatus::Refused;
                }
                _ => {}
            },
        }
    }
}

impl Drawable for Modal {
    fn draw(&self, f: &mut Frame, area: Rect) {
        if !self.is_open {
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

        match self.variant {
            ModalVariant::Confirmation => self.draw_confirm_modal(f, popup_wrapper),
            ModalVariant::Error | ModalVariant::Info => {
                self.draw_error_info_modal(f, popup_wrapper)
            }
            ModalVariant::Question(ref message) => {
                self.draw_question_modal(f, popup_wrapper, message);
            }
        }
    }
}

impl Modal {
    fn draw_error_info_modal(&self, f: &mut Frame, popup_wrapper: Rect) {
        self.draw_with_legend(f, popup_wrapper, vec!["Ok [y]".to_string()])
    }

    fn draw_confirm_modal(&self, f: &mut Frame, popup_wrapper: Rect) {
        self.draw_with_legend(
            f,
            popup_wrapper,
            vec!["Yes [y]".to_string(), "No [n]".to_string()],
        );
    }

    fn draw_question_modal(&self, f: &mut Frame, popup_wrapper: Rect, answer: &String) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ])
            .split(popup_wrapper);

        let question_wrapper = chunks[1];
        let answer_wrapper = chunks[2];

        let question_text = Paragraph::new(self.message.as_str())
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let answers_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(answer_wrapper);

        let answer_text = Paragraph::new(answer.as_str())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        Self::draw_modal_legend(
            vec![String::from("Ok [Enter]"), String::from("Cancel [Esc]")],
            chunks[3],
            f,
        );

        f.render_widget(Block::new().borders(Borders::all()), popup_wrapper);
        f.render_widget(question_text, question_wrapper);
        f.render_widget(answer_text, answers_chunks[1]);
    }

    fn draw_with_legend(&self, f: &mut Frame, popup_wrapper: Rect, legend: Vec<String>) {
        let v_segments = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
                Constraint::Percentage(30),
            ])
            .split(popup_wrapper);

        let question_wrapper = v_segments[1];

        let spacer_block = Block::new();
        let spacer = v_segments[2];

        let question_block = Block::new();
        let question = Paragraph::new(self.message.as_str())
            .centered()
            .block(question_block);

        Self::draw_modal_legend(legend, v_segments[3], f);

        f.render_widget(Block::new().borders(Borders::all()), popup_wrapper);
        f.render_widget(question, question_wrapper);
        f.render_widget(spacer_block, spacer);
    }

    fn draw_modal_legend(legend: Vec<String>, area: Rect, f: &mut Frame) {
        let answer_segments = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100 / legend.len() as u16)].repeat(legend.len()))
            .split(area);

        for (i, item) in legend.iter().enumerate() {
            let block = Block::new();
            let paragraph = Paragraph::new(item.as_str()).centered().block(block);
            f.render_widget(paragraph, answer_segments[i]);
        }
    }
}
