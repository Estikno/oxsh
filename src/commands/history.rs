pub enum HistoryArgs {
    /// Clear the history in memory
    Clear,
    /// Rewrite the history in memory with the history file
    Rewrite,
    /// Writes the history in memory to the history file
    Write,
    /// Saves a command in the history without excecuting it
    Save(String),
    /// Error handling args
    Error(String),
}

impl HistoryArgs {
    pub fn from_args(args: Vec<String>) -> HistoryArgs {
        if args.len() > 2 {
            return HistoryArgs::Error(String::from("Too many arguments! Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]"));
        }

        if args.len() == 0 {
            return HistoryArgs::Error(String::from("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]!"));
        }

        if args[0] == String::from("--clear") || args[0] == String::from("-c") {
            HistoryArgs::Clear
        }
        else if args[0] == String::from("--rewrite") || args[0] == String::from("-r") {
            HistoryArgs::Rewrite
        }
        else if args[0] == String::from("--write") || args[0] == String::from("-w") {
            HistoryArgs::Write
        }
        else if args[0] == String::from("--save") || args[0] == String::from("-save") {
            if args.len() < 2 {
                HistoryArgs::Error(String::from("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]"))
            }
            else {
                HistoryArgs::Save(args[1].clone())   
            }
        }
        else {
            HistoryArgs::Error(String::from("Use: history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]"))
        }
    }
}

pub fn history(args: Vec<String>) {
    
}