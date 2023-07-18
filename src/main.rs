#![allow(unused)]

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;
use std::fs::{self, create_dir, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use vcli::Args;

mod lang;

fn main() {
    let args = Args::parse();

    let language: &str = &args.language;
    let project_name = &args.project_name;

    if Path::new(project_name).exists() {
        println!("Target Project {} Already Exists!", project_name);
        return;
    }

    match language {
        "cpp" => lang::cpp::init_cpp(project_name),
        "shell" => lang::shell::init_shell(project_name),
        _ => println!(
            "No Support for {} yet, Please Check Docs For Correct Useage!",
            language
        ),
    };

    return;
}
