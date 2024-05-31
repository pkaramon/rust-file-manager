mod app;
mod binding;
mod command;
mod editor;
mod explorer_modal;
mod file_explorer;
mod legend;
mod text_editor;
mod window;

use anyhow::Result;
use app::App;
use command::InputHandler;
use crossterm::{
    event::{self, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn init() -> Result<Terminal<CrosstermBackend<io::Stdout>>, io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn exit(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<(), io::Error> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn main() -> Result<()> {
    let mut terminal = init().unwrap();

    let mut app = App::new()?;

    loop {
        let _ = terminal.draw(|f| {
            let _ = app.draw(f);
        });

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    let _ = app.handle_input(key.code);
                }
            }
        }

        if app.should_stop {
            break;
        }
    }

    Ok(exit(&mut terminal)?)
}
