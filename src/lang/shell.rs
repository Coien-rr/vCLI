use std::fs::{write, File};
use std::io::Write;
use std::path::Path;

pub fn hello_shell() {
    println!("Hello Shell!");
}

const FILE_TEMPLATE_JSON: &str = include_str!("../../templates/shell.json");

pub fn init_shell(file_name: &String) {
    if Path::new(file_name).exists() {
        println!("Target Shell File {} Already Exists!", file_name);
        return;
    }

    let file_path = Path::new(file_name).with_extension("sh");

    let mut file_writer = File::create(file_path).expect("File Create Error!");
    file_writer.write(b"#!/bin/bash");

    println!("Shell File {}.sh initialized successfully!", file_name);
}
