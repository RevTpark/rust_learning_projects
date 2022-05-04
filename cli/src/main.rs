use std::io;
use std::io::Write;
use std::fs;

fn main() {
    generate_dynamic();
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


