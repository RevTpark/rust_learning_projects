use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;

fn main() {
    let base_directory: String = String::from("C:\\Users\\tanis\\");
    let project_directories: Vec<String> = vec![String::from("PycharmProject"), String::from("VScode projects"), String::from("StudioProjects")];
    for directory in project_directories{
        for file in fs::read_dir(base_directory.clone()+&directory).unwrap(){
        println!("{}", file.unwrap().path().display());
        }
    }
    
    for files in fs::read_dir(base_directory.clone()+"PycharmProject\\ChatRoom\\backend").unwrap(){
        let path = files.unwrap().path();
        let ext = Path::new(&path).extension().and_then(OsStr::to_str);
        println!("{:?}", ext);
    }

    iterate_files(String::from("C:\\Users\\tanis\\PycharmProject\\ChatRoom"));
}

fn is_safe_to_iterate(filename: &std::ffi::OsStr) -> bool{
    let reg: Regex = Regex::new(r"^\.").unwrap();
    reg.is_match(&filename.to_str().unwrap()) || filename == "venv" || filename == "__pycache__" || filename == "migrations"
}

fn iterate_files(path: String){
    for file in fs::read_dir(path).unwrap(){
        let path = file.unwrap().path();
        let string_path = path.display().to_string();
        let temp = Path::new(&string_path).file_name().unwrap();
        if is_safe_to_iterate(temp){
            continue;
        }

        let ext = Path::new(&path).extension().and_then(OsStr::to_str);
        if ext == None{
            iterate_files(string_path);
            continue;
        }

        println!("{}", path.display());
    }
}
