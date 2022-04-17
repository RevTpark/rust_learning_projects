use std::io;
use std::io::Write;
// help, education, experiences, social, tech stack, projects, about me

fn main() {
    run_interpreter();
}

fn run_interpreter(){
    let base_directory: String = String::from("C:\\portfolio\\tanishq");
    loop{
        let mut input: String = String::new();
        print!("{}>", base_directory);
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input){
            Ok(_ok) => {
                match input.trim(){
                    "help" => some_func(),
                    "education" => some_func(),
                    "experience" => some_func(),
                    "social" => some_func(),
                    "skills" => some_func(),
                    "projects" => some_func(),
                    "about" => some_func(),
                    "quit" => break,
                    _ => println!("Invalid Command. Use 'help' for more information about commands.")
                }
            },
            Err(e) => println!("There was an error {}", e)
        }
    }
}

fn some_func(){
    println!("Running")
}