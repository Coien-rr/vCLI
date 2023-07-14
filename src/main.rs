#![allow(unused)]

use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use std::fs::{self, create_dir, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
struct Template {
    dir_info: FileTree,
    cmake_lists: Vec<String>,
    main_cpp: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct FileTree {
    include: Vec<String>,
    src: Vec<String>,
    tests: Vec<String>,
    app: Vec<String>,
    scripts: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run -- <template_file> <project_name>");
        return;
    }

    // 1. read template file
    let template_file = &args[1];
    let mut template_content = String::new();
    File::open(template_file)
        .and_then(|mut file| file.read_to_string(&mut template_content))
        .expect("Fail to to read template content");

    // 1.1 Serialize the content to json format
    let template: Template =
        serde_json::from_str(&template_content).expect("Failed to parse template JSON");

    let project_name = &args[2];
    if Path::new(project_name).exists() {
        println!("Target Project {} Already Exists!", project_name);
        return;
    }

    fs::create_dir(project_name).expect("Fail to create project dir");

    let project_path = Path::new(project_name);
    create_directory(&project_path, &template.dir_info.include);
    create_directory(&project_path, &template.dir_info.src);
    create_directory(&project_path, &template.dir_info.app);
    create_directory(&project_path, &template.dir_info.tests);

    let cmake_lists = template.cmake_lists.join("\n").replace("{}", project_name);
    let cmake_lists_path = project_path.join("CMakeLists.txt");
    let mut cmake_lists_file =
        File::create(cmake_lists_path).expect("Failing to Create CMakeLists.txt");

    cmake_lists_file
        .write_all(cmake_lists.as_bytes())
        .expect("Failed to write CMakeLists.txt");

    let main_cpp = template.main_cpp.join("\n").replace("{}", &project_name);
    let main_cpp_path = project_path.join("main.cc");
    let mut main_cpp_file = File::create(main_cpp_path).expect("ERROR: Failed to Create main.cc");
    main_cpp_file
        .write_all(main_cpp.as_bytes())
        .expect("ERROR: Failed to Write main.cc");

    println!("C++ project {} initialized successfully!", project_name);
}

fn create_directory(parent_path: &Path, directories: &[String]) {
    for dir in directories {
        let dir_path = parent_path.join(dir);
        fs::create_dir_all(&dir_path).expect("ERROR: Failed to create directory");
    }
}
