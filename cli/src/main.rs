use std::io;
use std::io::Write;
use std::fs;


fn main() {
    // run_interpreter();
    generate_dynamic();
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
                            // let result = execute_education(&base_directory);
                            // if result{
                            //     break
                            // }
                        },
                        "experience" => {
                            // let result = execute_experience(&base_directory);
                            // if result{
                            //     break
                            // }
                        },
                        "social" => {
                            // let result = execute_social(&base_directory);
                            // if result{
                            //     break
                            // }
                        },
                        "skills" => {
                            // let result = execute_skills(&base_directory);
                            // if result{
                            //     break
                            // }
                        },
                        "projects" => {
                            // let result = execute_projects(&base_directory);
                            // if result{
                            //     break
                            // }
                        },
                        "about" => {
                            // let result = execute_about(&base_directory);
                            // if result{
                            //     break
                            // }
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

fn generate_dynamic(){
    let base_directory: String = String::from("C:\\portfolio\\tanishq");

    let file = fs::File::open("src/text.json")
    .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be proper JSON");

    loop{
        let mut input: String = String::new();
        print!("{}>", base_directory);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input){
            Ok(_ok) => {
                let command = input.trim();
                if command == "quit"{
                    break;
                }
                else if command == "help"{ display_help() }
                else if command.starts_with("cd") {
                    let command: Vec<&str> = command.split(" ").collect();
                    let output = &json[&command[1]];
                    if output == &serde_json::Value::Null {
                        println!("Invalid Command. Use 'help' for more information about commands.")
                    }
                    else{
                        let result = execute_dynamic(&base_directory, command[1], output);
                        if result{
                            break;
                        }
                    }
                }
                else{
                    println!("Invalid Command. Use 'help' for more information about commands.")
                }
            },
            Err(_e) => {
                println!("Error while reading input");
            }
        }
    }    
    
}


fn execute_dynamic(base_url: &String, current: &str, commands: &serde_json::Value) -> bool{

    loop{
        let mut input: String = String::new();
        print!("{}\\{}>", base_url, current);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input){
            Ok(_ok) => {
                let cleaned_input = input.trim();
                match cleaned_input{
                    "help" => { display_help() },
                    "quit" => { return true; }
                    "cd .." => { return false; }
                    _ => {
                        let output = &commands[cleaned_input];
                        if output == &serde_json::Value::Null {
                            println!("Invalid Command. Use 'help' for more information about commands.")
                        }
                        else{
                            println!("{:?}", output.as_str());
                        }
                    }
                }
            },
            Err(_e) => {
                println!("Error while reading input");
            }
        }
    }
}


