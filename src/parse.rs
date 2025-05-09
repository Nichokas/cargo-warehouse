use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn copy_and_parse(file: PathBuf, destination: &Path) {
    let mut is_cargo_field: bool = false;
    let mut rs_file = File::create(destination.join("src").join("main.rs")).unwrap();
    let mut cargo_file = File::create(destination.join("Cargo.toml")).unwrap();
    
    // Write default Cargo.toml lines
    let default_cargo_lines = [
        "[package]",
        "name = \"rust_program\"",
        "version = \"1.0.0\"",
        "edition = \"2021\"",
        "",
    ];
    for line in default_cargo_lines.iter() {
        writeln!(cargo_file, "{}", line).unwrap();
    }

    // Copy --cargo lines from the rust file to the Cargo.toml
    for line in read_to_string(file).unwrap().lines() {
        if is_cargo_field {
            if line == "---" {
                is_cargo_field = false;
            } else {
                writeln!(cargo_file, "{}", line).unwrap();
            }
        } else if line == "---" || line == "---cargo" {
            is_cargo_field = true;
        } else {
            writeln!(rs_file, "{}", line).unwrap();
        }
    }

    drop(rs_file);
    drop(cargo_file);
}
