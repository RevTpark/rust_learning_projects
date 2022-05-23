use std::fs;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use regex::Regex;
use std::collections::HashMap;
mod constants;
use constants::GlobalConstants;

fn main(){
    search_files("al");
}

fn search_files(query: &str){
    let project_map: Vec<HashMap<u16, String>> = iterate_over_files();
    let mut filtered_projects: Vec<HashMap<u16, String>> = vec![];
    let reg: Regex = Regex::new(&format!(r"(?i:^.*{}.*$)", query)).unwrap();
    for projects in project_map{
        if reg.is_match(projects.get(&2).unwrap()){
            filtered_projects.push(projects);
        }
    }
    println!("{:?}", filtered_projects);
}


fn iterate_over_files() -> Vec<HashMap<u16, String>>{
    let base_directory: String = GlobalConstants::new().base_directory;
    let mut final_projects_list: Vec<HashMap<u16, String>> = vec![];
    let project_directories: Vec<String> = GlobalConstants::new().project_directories;
    for directory in project_directories{
        for file in fs::read_dir(base_directory.clone()+&directory).unwrap(){
            let mut langs: HashMap<String, u16> = HashMap::new();
            let path: PathBuf = file.unwrap().path();
            let string_path: String = path.display().to_string();
            iterate_files(string_path, &mut langs);
            let mut max_name: String = String::new(); 
            let mut max_points: u16 = 0;
            for (language, points) in &langs{
                if *points > max_points{
                    max_points = *points;
                    max_name = language.clone();
                }
            }
            let new_project: HashMap<u16, String> = HashMap::from([
                (0, path.display().to_string()),
                (1, max_name),
                (2, path.file_name().unwrap().to_str().unwrap().to_string())
            ]);
            final_projects_list.push(new_project);
        }
    }

    // println!("{:?}", final_projects_list);
    // println!("{}", final_projects_list.len());
    return final_projects_list;
}

fn is_safe_to_iterate(filename: &std::ffi::OsStr) -> bool{
    let restricted: Vec<&str> = vec!["andriod", "build", "ios", "target", "ckeditor", "material-dashboard", "node_modules", "venv", "__pycache__", "migrations"];
    let reg: Regex = Regex::new(r"^\.").unwrap();
    reg.is_match(&filename.to_str().unwrap()) || restricted.contains(&filename.to_str().unwrap())
}

fn iterate_files(path: String, langs: &mut HashMap<String, u16>){
    let files = fs::read_dir(&path);
    match &files{
        Ok(_) => {
            for file in files.unwrap(){
                let path: PathBuf = file.unwrap().path();
                let string_path: String = path.display().to_string();
                let temp = Path::new(&string_path).file_name().unwrap();
                if is_safe_to_iterate(temp){
                    continue;
                }
                
                match Path::new(&path).extension().and_then(OsStr::to_str){
                    None => {
                        iterate_files(string_path, langs);
                        continue;
                    },
                    Some(text) => {
                        let ext: &str = text;
                        let allowed_exts: Vec<&str> = vec!["png", "jpeg", "jpg", "class", "json", "exe", "pdf"];
                        if !allowed_exts.contains(&ext){
                            if langs.contains_key(ext){
                                let counter = langs.entry(String::from(ext)).or_insert(0);
                                *counter += 1;
                            }
                            else{
                                langs.insert(String::from(ext), 1);
                            }
                        }
                        
                    }
                }

                // println!("{}", path.display());
            }
        },
        Err(_) => {
            // println!("Illegal File Encountered booom!! {}", path);
        },
    }
}
