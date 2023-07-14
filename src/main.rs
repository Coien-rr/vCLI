#![allow(unused)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs::{self, create_dir, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug)]
struct Template {
    dir_info: FileTree,
    cmake_lists: Vec<String>,
    main_cpp: Vec<String>,
    run_sh: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct FileTree {
    include: Vec<String>,
    src: Vec<String>,
    tests: Vec<String>,
    app: Vec<String>,
    scripts: Vec<String>,
}

const FILE_TREE_JSON: &str = include_str!("../templates/cpp.json");

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: vCLI <project_name>");
        return;
    }

    let template: Template =
        serde_json::from_str(&FILE_TREE_JSON).expect("Failed to parse template JSON");

    let project_name = &args[1];
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
    create_directory(&project_path, &template.dir_info.scripts);

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

    let run_sh = template.run_sh.join("\n");
    let run_sh_path = project_path
        .join(&template.dir_info.scripts[0])
        .join("run.sh");
    let mut run_sh_file =
        File::create(run_sh_path).expect("ERROR: Failed to create scripts/run.sh");
    run_sh_file
        .write_all(run_sh.as_bytes())
        .expect("ERROR: Failed to Write run.sh");

    println!("C++ project {} initialized successfully!", project_name);
}

fn create_directory(parent_path: &Path, directories: &[String]) {
    for dir in directories {
        let dir_path = parent_path.join(dir);
        fs::create_dir_all(&dir_path).expect("ERROR: Failed to create directory");
    }
}
