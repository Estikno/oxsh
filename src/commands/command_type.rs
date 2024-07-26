// TODO: Add arguments to each command that needs it. Lifetimes are needed for args.
pub enum CommandType {
    CD(Vec<String>),
    Help,
    Exit,
    External(String, Vec<String>),
}

impl CommandType {
    pub fn from_str(command: &str, args: Vec<String>) -> CommandType {
        match command {
            "cd" => CommandType::CD(args),
            "help" => CommandType::Help,
            "exit" => CommandType::Exit,
            com => CommandType::External(com.to_string(), args),
        }
    }
}