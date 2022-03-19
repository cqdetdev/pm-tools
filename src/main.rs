use std::env::args;

mod command_handler;
mod commands;
mod constants;
use command_handler::CommandHandler;

pub fn main() {
    let args: Vec<String> = args().collect();
    let mut h = CommandHandler::new();

    h.register("help", Box::new(commands::help));
    h.register("makeserver", Box::new(commands::makeserver));
    if args.get(1).is_none() {
        let help = h.get("help").unwrap();
        help(args);
    } else {
        let cmd = h.get(args.get(1).unwrap()).unwrap();
        cmd(args);
    }
}
