use crate::commands::history::HistoryArgs;

pub enum CommandType {
    CD(Vec<String>),
    Help,
    History(HistoryArgs),
    Exit,
    External(String, Vec<String>),
}

impl CommandType {
    pub fn from_str(command: &str, args: Vec<String>) -> CommandType {
        match command {
            "cd" => CommandType::CD(args),
            "help" => CommandType::Help,
            "history" => CommandType::History(HistoryArgs::from_args(args)),
            "exit" => CommandType::Exit,
            com => CommandType::External(com.to_string(), args),
        }
    }
}