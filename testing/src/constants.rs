
#[derive(Debug)]
pub struct GlobalConstants{
    pub base_directory: String,
    pub project_directories: Vec<String>,
}

impl GlobalConstants{
    pub fn new() -> Self{
        GlobalConstants{
            base_directory: String::from("C:\\Users\\tanis\\"),
            project_directories: vec![String::from("PycharmProject"), String::from("StudioProjects"), String::from("VScode projects")]
        }
    }
}


