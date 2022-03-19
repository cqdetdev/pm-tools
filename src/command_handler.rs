use std::collections::HashMap;

pub type CommandFn = dyn Fn(Vec<String>);

pub struct CommandHandler {
    pub commands: HashMap<String, Box<CommandFn>>,
}

impl CommandHandler {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register(&mut self, command: &str, cmd_fn: Box<CommandFn>) {
        self.commands.insert(command.to_owned(), cmd_fn);
    }

    pub fn get(&self, command: &str) -> Option<&Box<CommandFn>> {
        self.commands.get(&command.to_owned())
    }

    pub fn has(&self, command: &str) -> bool {
        self.get(command).is_some()
    }
}
