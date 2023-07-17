#![allow(unused)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{self, write, File};
use std::io::{self, Write};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about = "A tiny CLI for Init project")]
pub struct Args {
    /// the target project name
    pub project_name: String,
    #[arg(short, long, default_value_t = String::from("cpp"))]
    pub language: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Template {
    pub dir_list: Vec<String>,
    pub file_contents: FileContents,
    pub dir_info: FileTree,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileTree {
    include: Vec<String>,
    src: Vec<String>,
    tests: Vec<String>,
    app: Vec<String>,
    scripts: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileContents {
    cmakelist_txt: Vec<String>,
    main_cc: Vec<String>,
    src_cmakelists_txt: Vec<String>,
    src_hello_cc: Vec<String>,
    include_hello_h: Vec<String>,
    scripts_run_sh: Vec<String>,
}

pub fn get_dir_lists<'a>(filetree: &'a FileTree, dir: String) -> &'a [String] {
    match dir.as_str() {
        "include" => &filetree.include,
        "src" => &filetree.src,
        "tests" => &filetree.tests,
        "app" => &filetree.app,
        "scripts" => &filetree.scripts,
        _ => panic!("Invalid directory"),
    }
}

fn create_directory(parent_path: &Path, directories: &[String]) -> io::Result<()> {
    for dir in directories {
        let dir_path = parent_path.join(dir);
        fs::create_dir_all(&dir_path).expect("ERROR: Failed to create directory");
    }

    Ok(())
}

pub fn generate_dir(template: &Template, project_path: &Path) -> io::Result<()> {
    for dir in &template.dir_list {
        let target = dir.clone();
        match target.as_str() {
            "include" => create_directory(project_path, &template.dir_info.include),
            "src" => create_directory(project_path, &template.dir_info.src),
            "tests" => create_directory(project_path, &template.dir_info.tests),
            "app" => create_directory(project_path, &template.dir_info.app),
            "scripts" => create_directory(project_path, &template.dir_info.scripts),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "Dir Create ERROR!")),
        };
    }

    Ok(())
}

fn write_file(file_path: &Path, contents: &[String]) -> io::Result<()> {
    let mut target = File::create(file_path)?;
    for line in contents {
        target.write_all(line.as_bytes())?;
        target.write_all(b"\n")?;
    }
    Ok(())
}

pub fn generate_files(
    project_path: &Path,
    file_list: &[String],
    file_template: &FileContents,
) -> io::Result<()> {
    for file in file_list {
        let content = convert_to_snake_case(&file);
        let file_path = project_path.join(&file);
        match content.as_str() {
            "cmakelists_txt" => write_file(&file_path, &file_template.cmakelist_txt),
            "main_cc" => write_file(&file_path, &file_template.main_cc),
            "src_cmakelists_txt" => write_file(&file_path, &file_template.src_cmakelists_txt),
            "src_hello_cc" => write_file(&file_path, &file_template.src_hello_cc),
            "include_hello_h" => write_file(&file_path, &file_template.include_hello_h),
            "scripts_run_sh" => write_file(&file_path, &file_template.scripts_run_sh),
            _ => return Err(io::Error::new(io::ErrorKind::Other, "File Create ERROR!")),
        };
    }

    Ok(())
}

fn convert_to_snake_case(input: &str) -> String {
    let mut output = String::new();

    for (i, c) in input.chars().enumerate() {
        if c == '/' || c == '.' {
            if i > 0 {
                output.push('_');
            }
        } else {
            output.push(c.to_ascii_lowercase());
        }
    }

    output
}

