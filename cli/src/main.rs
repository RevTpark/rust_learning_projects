use std::io;
use std::io::Write;
// help, education, experiences, social, tech stack, projects, about me

// "education" => some_func(),
// "experience" => some_func(),
// "social" => some_func(),
// "skills" => some_func(),
// "projects" => some_func(),
// "about" => some_func(),

// Push location in stack
// the current stack location commands should now be activated
// Popping the stack on cd ..

fn main() {
    run_interpreter();
}

fn run_interpreter(){
    let base_directory: String = String::from("C:\\portfolio\\tanishq");
    let directories: Vec<&str> = vec!["education", "experiences", "social", "skills", "projects", "about"];
    loop{
        let mut input: String = String::new();
        print!("{}>", base_directory);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input){
            Ok(_ok) => {
                let command = input.trim();
                if command == "help"{ display_help() }
                else if command == "quit"{ break }
                else if command.starts_with("cd"){ 
                    let command_items: Vec<&str> = command.split(" ").collect();
                    let location: &str = command_items[1];
                    if directories.contains(&location){
                        // or we can have a different loop systems for each type of command, enums can be used.
                    }else{
                        println!("Invalid Command. Use 'help' for more information about commands.")
                    }
                }
                else { println!("Invalid Command. Use 'help' for more information about commands.") }
            },
            Err(e) => println!("There was an error {}", e)
        }
    }
}

fn display_help(){
    println!("These are all the commands and help you need ok!")
}