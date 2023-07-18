use std::fs;
use std::path::Path;

use vcli::{generate_dir, generate_files, get_dir_lists, FileTree, Template};

const FILE_TREE_JSON: &str = include_str!("../../templates/cpp.json");

pub fn hello_cpp() {
    println!("Hello Cpp!");
}

pub fn init_cpp(project_name: &String) {
    let mut template: Template =
        serde_json::from_str(&FILE_TREE_JSON).expect("Failed to parse template JSON");

    fs::create_dir(project_name).expect("Fail to create project dir");

    let project_path = Path::new(project_name);

    if let Err(err) = generate_dir(&template, &project_path) {
        eprintln!("ERROR generate_dir {}", err);
    }

    if let Err(err) = generate_files(
        &project_path,
        project_name.clone(),
        &template.file_list,
        &mut template.file_contents,
    ) {
        eprintln!("Error writing to file: {}", err);
    }

    println!("C++ project {} initialized successfully!", project_name);
}
