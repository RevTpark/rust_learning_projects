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
                    // or we can have a different loop systems for each type of command, enums can be used.
                    match location{
                        "education" => {
                            let result = execute_education(&base_directory);
                            if result{
                                break
                            }
                        },
                        "experience" => {
                            let result = execute_experience(&base_directory);
                            if result{
                                break
                            }
                        },
                        "social" => {
                            let result = execute_social(&base_directory);
                            if result{
                                break
                            }
                        },
                        "skills" => {
                            let result = execute_skills(&base_directory);
                            if result{
                                break
                            }
                        },
                        "projects" => {
                            let result = execute_projects(&base_directory);
                            if result{
                                break
                            }
                        },
                        "about" => {
                            let result = execute_about(&base_directory);
                            if result{
                                break
                            }
                        },
                        _ => println!("Invalid Command. Use 'help' for more information about commands.")
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

fn execute_education(base_directory: &String) -> bool{
    let current_directory: String = base_directory.clone() + "\\education";
    loop{
        let mut input: String = String::new();
        print!("{}>", current_directory);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input){
            Ok(_ok) => {
                let cmdlet = input.trim();
                match cmdlet{
                    "help" => display_help(),
                    "quit" => {
                        return true;
                    },
                    "cd .." => {
                        return false;
                    }
                    "SSC" => println!("This is SSC"),
                    "HSC" => println!("This is HSC"),
                    "current" => println!("This is current"),
                    _ => println!("Invalid command.")
                }
            },
            Err(e) => println!("There was an error {}", e)
        }
    }
}


fn execute_experience(base_directory: &String) -> bool{
    true
}

fn execute_social(base_directory: &String) -> bool{
    true
}

fn execute_skills(base_directory: &String) -> bool{
    true
}

fn execute_projects(base_directory: &String) -> bool{
    true
}

fn execute_about(base_directory: &String) -> bool{
    true
}
