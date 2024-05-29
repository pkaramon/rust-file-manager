use crate::app::App;

pub struct Command {
    pub id: &'static str,
    pub name: &'static str,
    pub func: fn(&mut App) -> (),
}

pub fn get_commands() -> Vec<Command> {
    vec![
        Command {
            id: "quit",
            name: "Quit",
            func: App::quit,
        },
        Command {
            id: "select_previous_file",
            name: "Prev file",
            func: App::select_previous_file,
        },
        Command {
            id: "select_next_file",
            name: "Next file",
            func: App::select_next_file,
        },
        Command {
            id: "open_selected_file",
            name: "Open file",
            func: App::open_selected_file,
        },
        Command {
            id: "go_back",
            name: "Back",
            func: App::go_back,
        },
    ]
}
