use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::fs;
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

fn generate_dynamic(){
    let file = fs::File::open("src/text.json")
    .expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
    .expect("file should be proper JSON");
    

    let mut intial_commands: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut temp: HashMap<String, String> = HashMap::new();
    temp.insert(String::from("SSC"), String::from("This is SSC"));
    temp.insert(String::from("HSC"), String::from("This is HSC"));
    temp.insert(String::from("current"), String::from("This is Current"));

    intial_commands.insert(String::from("education"), temp);
 
    
    let input: String = String::from("skills");
    if intial_commands.contains_key(&input){
        match intial_commands.get(&input){
            Some(value) => {
                let result = execute_dynamic(String::from("C:\\portfolio\\tanishq"),input, value);
                // if result{ break }
            }
            None => {
                println!("Nothing found here")
            }
        }
        
    }
    else{
        println!("Invalid command");
    }
}


fn execute_dynamic(base_url: String, current: String, commands: &HashMap<String, String>) -> bool{
    println!("{}\\{}>", base_url, current);
    println!("{:?}", commands);
    let input: String = String::from("Skill 1");
    println!("{}", commands.contains_key(&input));

    true
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
