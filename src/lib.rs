use std::fs::{read_to_string, File};
use std::io::Write;
use walkdir::WalkDir;

/// Creates a md file of the project at the given directory, and then exports at the given output
pub fn to_md(file_path: String, output_path : String, extension : String) -> String {
    let mut return_string = String::new();
    for dir in WalkDir::new(file_path.clone()) {
        let dir = dir.unwrap();
        let path = dir.path();
        if path.to_str().unwrap().ends_with(&extension) {

            let split = path.to_str().unwrap().split(&file_path).collect::<Vec<&str>>();
            let file_name = split.get(1).unwrap();

            if file_name.starts_with("target") ||  file_name.starts_with("\\target") {continue}

            println!("{}",file_name);

            return_string = format!("{}\n# {}\n",return_string, file_name);

            let code = read_to_string(path.clone()).unwrap();
            let output = format!("```{}\n{}\n```\n", extension.strip_prefix(".").unwrap(),code);

            return_string = format!("{}\n{}",return_string, output);

        }
    }

    let binding = file_path.replace("/", "\\");
    let split_path =  binding.split("\\").collect::<Vec<&str>>();
    let mut name = String::new();
    if split_path.last().is_some(){
        name = split_path.last().unwrap().to_string();
    }
    else {
        let index = split_path.len()-2usize;
        name = split_path.get(index).unwrap().to_string();
    }
    let mut output_file = File::create(format!("{}\\{}.md",file_path.clone(), name));
    output_file.unwrap().write_all(return_string.as_bytes());
    format!("output md file at: {}", format!("{}\\{}.md",file_path.clone(), name))
}