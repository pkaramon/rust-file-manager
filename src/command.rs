use crossterm::event::KeyCode;

use crate::binding::get_bindings;

pub struct Command<T> {
    pub id: &'static str,
    pub name: &'static str,
    pub func: fn(&mut T, KeyCode) -> bool,
}

pub trait CommandHandler: Sized {
    fn get_name(&self) -> &'static str;
    fn get_commands(&self) -> Vec<Command<Self>>;

    fn handle(&mut self, key_code: KeyCode) -> bool {
        let name = self.get_name();
        let bindings = get_bindings();

        let binding_option = bindings.iter().find(|binding| {
            let command_id_parts: Vec<&str> = binding.command_id.split(".").collect();
            let handler_name = command_id_parts.first().unwrap().to_owned();
            handler_name == name && binding.key_code == key_code
        });
        if binding_option.is_some() {
            let binding = binding_option.unwrap();
            let commands = self.get_commands();
            let command_id = binding.command_id;
            let command_option = commands.iter().find(|command| command.id == command_id);
            if command_option.is_some() {
                let command = command_option.unwrap();
                (command.func)(self, binding.key_code)
            } else {
                false
            }
        } else {
            false
        }
    }
}
