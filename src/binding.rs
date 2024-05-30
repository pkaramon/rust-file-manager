use crossterm::event::KeyCode;

#[derive(Copy, Clone)]
pub struct Binding {
    pub command_id: &'static str,
    pub key_code: KeyCode,
}

pub fn get_bindings() -> Vec<Binding> {
    vec![
        Binding {
            command_id: "app.quit",
            key_code: KeyCode::Char('q'),
        },
        Binding {
            command_id: "app.open_selected_file",
            key_code: KeyCode::Enter,
        },
        Binding {
            command_id: "app.go_back",
            key_code: KeyCode::Esc,
        },
        Binding {
            command_id: "explorer.select_previous_file",
            key_code: KeyCode::Up,
        },
        Binding {
            command_id: "explorer.select_next_file",
            key_code: KeyCode::Down,
        },
        Binding {
            command_id: "explorer.open_selected_file",
            key_code: KeyCode::Enter,
        },
        Binding {
            command_id: "explorer.go_back",
            key_code: KeyCode::Esc,
        },
        Binding {
            command_id: "text_editor.next_char",
            key_code: KeyCode::Right,
        },
        Binding {
            command_id: "text_editor.prev_char",
            key_code: KeyCode::Left,
        },
        Binding {
            command_id: "text_editor.next_line",
            key_code: KeyCode::Down,
        },
        Binding {
            command_id: "text_editor.prev_line",
            key_code: KeyCode::Up,
        },
    ]
}
