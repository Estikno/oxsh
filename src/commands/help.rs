pub fn help() {
    println!("  _____  ______  _   _");
    println!(" / _ \\ \\/ / ___|| | | |");
    println!("| | | \\  /\\___ \\| |_| |");
    println!("| |_| /  \\ ___) |  _  |");
    println!(" \\___/_/\\_\\____/|_| |_|");
    println!("");
    println!("oxsh - a minimalist shell");
    println!("");
    println!("Built-in commands:");
    println!("- cd <directory>: Change the current directory");
    println!("- history [--clear | -c] [--rewrite | -r] [--write | -w] [--save | -s <command>]: manages the command history");
    println!("- help: Display this help message");
    println!("- exit: Exit the shell");
}