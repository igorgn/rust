use crate::commands::Commands;

pub fn map_command(command: &str) -> Commands {
    match command {
        "ls" => Commands::Ls,
        "cat" => Commands::Cat,
        "echo" => Commands::Echo,
        "grep" => Commands::Grep,
        "exit" => Commands::Exit,
        _ => Commands::Unknown(command.to_string()),
    }
}
