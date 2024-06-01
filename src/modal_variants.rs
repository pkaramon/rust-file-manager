use crossterm::event::KeyCode;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::modal::{ModalState, ModalVariant};

pub struct InfoVariant {
    message: String,
}

impl InfoVariant {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}

impl ModalVariant for InfoVariant {
    fn handle_input(&mut self, state: &mut ModalState, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('y') => {
                state.is_open = false;
            }
            _ => {}
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        draw_with_legend(&self.message, f, area, vec!["Ok [y]".to_string()]);
    }
}

pub struct QuestionVariant {
    message: String,
    answer: String,
    on_confirm: ModalCallback<String>,
}

impl QuestionVariant {
    pub fn new(message: String, answer: String, on_confirm: ModalCallback<String>) -> Self {
        Self {
            message,
            answer,
            on_confirm,
        }
    }
}

impl ModalVariant for QuestionVariant {
    fn handle_input(&mut self, state: &mut ModalState, key_code: KeyCode) {
        match key_code {
            KeyCode::Backspace => {
                self.answer.pop();
            }
            KeyCode::Char(c) => {
                self.answer.push(c);
            }
            KeyCode::Enter => {
                state.is_open = false;

                let on_confirm = &mut self.on_confirm;
                (on_confirm)(self.answer.clone())
            }
            KeyCode::Esc => {
                state.is_open = false;
            }
            _ => {}
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(20),
                Constraint::Percentage(10),
            ])
            .split(area);

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

        let answer_text = Paragraph::new(self.answer.as_str())
            .block(Block::default().borders(Borders::ALL))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        draw_modal_legend(
            vec![String::from("Ok [Enter]"), String::from("Cancel [Esc]")],
            chunks[3],
            f,
        );

        f.render_widget(Block::new().borders(Borders::all()), area);
        f.render_widget(question_text, question_wrapper);
        f.render_widget(answer_text, answers_chunks[1]);
    }
}

type ModalCallback<T = ()> = Box<dyn Fn(T)>;

pub struct ConfirmationVariant {
    message: String,
    on_confirm: ModalCallback,
}

impl ConfirmationVariant {
    pub fn new(message: String, on_confirm: ModalCallback) -> Self {
        Self {
            message,
            on_confirm,
        }
    }
}

impl ModalVariant for ConfirmationVariant {
    fn handle_input(&mut self, state: &mut ModalState, key_code: KeyCode) {
        match key_code {
            KeyCode::Char('y') => {
                state.is_open = false;
                (self.on_confirm)(());
            }
            KeyCode::Char('n') => {
                state.is_open = false;
            }
            _ => {}
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        draw_with_legend(
            &self.message,
            f,
            area,
            vec!["Yes [y]".to_string(), "No [n]".to_string()],
        );
    }
}

pub struct OptionsVariant {
    message: String,
    options: Vec<String>,
    selected_index: usize,
    on_press: ModalCallback<usize>,
}

impl OptionsVariant {
    pub fn new(message: String, options: Vec<String>, on_press: ModalCallback<usize>) -> Self {
        Self {
            message,
            options,
            selected_index: 0,
            on_press,
        }
    }
}

impl ModalVariant for OptionsVariant {
    fn handle_input(&mut self, state: &mut ModalState, key_code: KeyCode) {
        match key_code {
            KeyCode::Char(c) => {
                let index: usize = c.to_string().parse::<usize>().unwrap_or(0);
                if index > 0 && index <= self.options.len() {
                    state.is_open = false;
                    self.selected_index = index - 1;
                    (self.on_press)(self.selected_index);
                }
            }
            KeyCode::Esc => {
                state.is_open = false;
            }
            _ => {}
        }
    }

    fn draw(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Fill(2),
                Constraint::Fill(2),
                Constraint::Fill(2),
                Constraint::Fill(1),
            ])
            .split(area);

        let question_wrapper = chunks[1];
        let options_wrapper = chunks[2];

        let question_text = Paragraph::new(self.message.as_str())
            .block(Block::default())
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        let option_texts = self
            .options
            .iter()
            .enumerate()
            .map(|(i, option)| format!("[{}]. {}", i + 1, option))
            .map(|text| ListItem::new(Text::from(text).alignment(Alignment::Center)));

        let list = List::new(option_texts);

        draw_modal_legend(vec![String::from("Cancel [Esc]")], chunks[3], f);

        f.render_widget(Block::new().borders(Borders::all()), area);
        f.render_widget(question_text, question_wrapper);
        f.render_widget(list, options_wrapper);
    }
}

fn draw_with_legend(message: &String, f: &mut Frame, popup_wrapper: Rect, legend: Vec<String>) {
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
    let question = Paragraph::new(message.as_str())
        .centered()
        .block(question_block);

    draw_modal_legend(legend, v_segments[3], f);

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
