use crossterm::event::KeyCode;

pub struct Binding {
    pub key_code: KeyCode,
    pub command_id: &'static str,
}

pub fn get_bindings() -> Vec<Binding> {
    vec![
        Binding {
            key_code: KeyCode::Char('q'),
            command_id: "quit",
        },
        Binding {
            key_code: KeyCode::Up,
            command_id: "select_previous_file",
        },
        Binding {
            key_code: KeyCode::Down,
            command_id: "select_next_file",
        },
        Binding {
            key_code: KeyCode::Enter,
            command_id: "open_selected_file",
        },
        Binding {
            key_code: KeyCode::Esc,
            command_id: "go_back",
        },
    ]
}
