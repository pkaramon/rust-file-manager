use crossterm::event::KeyCode;

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
            command_id: "explorer.select_previous_file",
            key_code: KeyCode::Char('k'),
        },
        Binding {
            command_id: "explorer.select_next_file",
            key_code: KeyCode::Char('j'),
        },
        Binding {
            command_id: "explorer.open_selected_file",
            key_code: KeyCode::Enter,
        },
        Binding {
            command_id: "explorer.delete_current_file",
            key_code: KeyCode::Char('d'),
        },
        Binding {
            command_id: "explorer.move_current_file",
            key_code: KeyCode::Char('m'),
        },
        Binding {
            command_id: "explorer.filter",
            key_code: KeyCode::Char('/'),
        },
        Binding {
            command_id: "explorer.go_back",
            key_code: KeyCode::Esc,
        },
        Binding {
            command_id: "text_editor.next_char",
            key_code: KeyCode::Char('l'),
        },
        Binding {
            command_id: "text_editor.prev_char",
            key_code: KeyCode::Char('h'),
        },
        Binding {
            command_id: "text_editor.next_line",
            key_code: KeyCode::Char('j'),
        },
        Binding {
            command_id: "text_editor.prev_line",
            key_code: KeyCode::Char('k'),
        },
        Binding {
            command_id: "text_editor.save",
            key_code: KeyCode::Char('s'),
        },
        Binding {
            command_id: "text_editor.insert_mode",
            key_code: KeyCode::Char('i'),
        },
        Binding {
            command_id: "text_editor.go_back",
            key_code: KeyCode::Esc,
        },
    ]
}
