#![allow(unused)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs::{self, create_dir, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use vcli::{generate_dir, generate_files, get_dir_lists, Args, FileTree, Template};

const FILE_TREE_JSON: &str = include_str!("../templates/cpp.json");

fn main() {
    let args = Args::parse();

    let template: Template =
        serde_json::from_str(&FILE_TREE_JSON).expect("Failed to parse template JSON");

    let project_name = &args.project_name;
    if Path::new(project_name).exists() {
        println!("Target Project {} Already Exists!", project_name);
        return;
    }

    fs::create_dir(project_name).expect("Fail to create project dir");

    let project_path = Path::new(project_name);

    if let Err(err) = generate_dir(&template, &project_path) {
        eprintln!("ERROR generate_dir {}", err);
    }

    if let Err(err) = generate_files(&project_path, &template.dir_list, &template.file_contents) {
        eprintln!("Error writing to file: {}", err);
    }

    println!("C++ project {} initialized successfully!", project_name);
}

fn write_file(file_path: &Path, contents: &[String]) -> io::Result<()> {
    let mut target = File::create(file_path)?;
    for line in contents {
        target.write_all(line.as_bytes())?;
        target.write_all(b"\n")?;
    }
    Ok(())
}
