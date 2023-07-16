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
    src_cmake: Vec<String>,
    main_cpp: Vec<String>,
    include_hello: Vec<String>,
    src_hello: Vec<String>,
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

    let cmake_lists = template.cmake_lists.join("\n").replace("{}", &project_name);
    let cmake_lists_path = project_path.join("CMakeLists.txt");
    if let Err(err) = write_file(&cmake_lists_path, &[cmake_lists.clone()]) {
        eprintln!("Error writing to file: {}", err);
    }

    let src_cmake_path = project_path
        .join(&template.dir_info.src[0])
        .join("CMakeLists.txt");
    if let Err(err) = write_file(&src_cmake_path, &template.src_cmake) {
        eprintln!("Error writing to file: {}", err);
    }

    let include_hello_path = project_path
        .join(&template.dir_info.include[0])
        .join("hello.h");
    if let Err(err) = write_file(&include_hello_path, &template.include_hello) {
        eprintln!("Error writing to file: {}", err);
    }

    let src_hello_path = project_path
        .join(&template.dir_info.src[0])
        .join("hello.cc");
    if let Err(err) = write_file(&src_hello_path, &template.src_hello) {
        eprintln!("Error writing to file: {}", err);
    }

    // let main_cpp = template.main_cpp.join("\n").replace("{}", &project_name);
    let main_cpp_path = project_path.join("main.cc");
    if let Err(err) = write_file(&main_cpp_path, &template.main_cpp) {
        eprintln!("Error writing to file: {}", err);
    }

    let run_sh_path = project_path
        .join(&template.dir_info.scripts[0])
        .join("run.sh");

    if let Err(err) = write_file(&run_sh_path, &template.run_sh) {
        eprintln!("Error writing to file: {}", err);
    }

    println!("C++ project {} initialized successfully!", project_name);
}

fn create_directory(parent_path: &Path, directories: &[String]) {
    for dir in directories {
        let dir_path = parent_path.join(dir);
        fs::create_dir_all(&dir_path).expect("ERROR: Failed to create directory");
    }
}

fn write_file(file_path: &Path, contents: &[String]) -> io::Result<()> {
    let mut target = File::create(file_path)?;
    for line in contents {
        target.write_all(line.as_bytes())?;
        target.write_all(b"\n")?;
    }
    Ok(())
}
