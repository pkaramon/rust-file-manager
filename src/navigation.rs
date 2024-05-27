use crossterm::event::KeyCode;

use crate::app::{ActiveWindow, App};

pub fn handle_input(app: &mut App, key_code: KeyCode) {
    match key_code {
        KeyCode::Char('q') => {
            app.should_stop = true;
        }
        KeyCode::Up => {
            if app.active_window == ActiveWindow::Explorer {
                app.explorer.select_previous();
            }
        }
        KeyCode::Down => {
            if app.active_window == ActiveWindow::Explorer {
                app.explorer.select_next();
            }
        }
        KeyCode::Enter => {
            if app.active_window == ActiveWindow::Explorer {
                app.open_selected_file();
            }
        }
        KeyCode::Esc => {
            if app.active_window == ActiveWindow::Explorer {
                app.explorer.go_back();
            } else {
                app.activate_explorer();
            }
        }
        _ => {}
    }
}
