use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashSet;
use regex::Regex;

pub fn copy_and_parse(file: PathBuf, destination: &Path) {
    let mut rs_file = File::create(destination.join("src").join("main.rs")).unwrap();
    let mut cargo_file = File::create(destination.join("Cargo.toml")).unwrap();
    let original_script_dir = file.parent().unwrap_or_else(|| Path::new("."));
    
    // Get file content
    let content = read_to_string(&file).unwrap();
    println!("Original script dir: {}", original_script_dir.display());
    
    // Extract cargo section from the content
    let mut cargo_section = String::new();
    let mut in_cargo = false;
    
    for line in content.lines() {
        if line == "---" && in_cargo {
            in_cargo = false;
        } else if (line == "---" || line == "---cargo") && !in_cargo {
            in_cargo = true;
        } else if in_cargo {
            cargo_section.push_str(line);
            cargo_section.push('\n');
        }
    }
    
    // Find all relative path dependencies using a simple pattern matching approach
    let re = Regex::new(r#"path\s*=\s*"([^"]+)""#).unwrap();
    let mut relative_paths = HashSet::new();
    
    for cap in re.captures_iter(&cargo_section) {
        let path_str = cap.get(1).unwrap().as_str();
        let rel_path = PathBuf::from(path_str);
        if rel_path.is_relative() {
            let abs_path = original_script_dir.join(&rel_path).canonicalize().unwrap_or_else(|_| {
                let abs_path = original_script_dir.join(&rel_path);
                println!("Warning: Could not canonicalize path: {}", abs_path.display());
                abs_path
            });
            println!("Found relative path: {} -> {}", rel_path.display(), abs_path.display());
            if abs_path.exists() {
                println!("Path exists! Adding to symlinks list.");
                relative_paths.insert((rel_path, abs_path));
            } else {
                println!("Warning: Path not found: {}", abs_path.display());
            }
        }
    }
    
    println!("Creating {} symlinks...", relative_paths.len());
    
    // Create symlinks for each relative dependency
    for (rel_path, abs_path) in &relative_paths {
        let target_path = destination.join(rel_path);
        
        // Create parent directories if needed
        if let Some(parent) = target_path.parent() {
            println!("Creating directory: {}", parent.display());
            fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Warning: Could not create directory {}: {}", parent.display(), e);
            });
        }
        
        println!("Creating symlink: {} -> {}", target_path.display(), abs_path.display());
        
        // Create symlink
        #[cfg(unix)]
        std::os::unix::fs::symlink(abs_path, &target_path).unwrap_or_else(|e| {
            eprintln!("Warning: Could not create symlink from {} to {}: {}", 
                      abs_path.display(), target_path.display(), e);
        });
        
        #[cfg(windows)]
        if abs_path.is_dir() {
            std::os::windows::fs::symlink_dir(abs_path, &target_path).unwrap_or_else(|e| {
                eprintln!("Warning: Could not create directory symlink from {} to {}: {}", 
                          abs_path.display(), target_path.display(), e);
            });
        } else {
            std::os::windows::fs::symlink_file(abs_path, &target_path).unwrap_or_else(|e| {
                eprintln!("Warning: Could not create file symlink from {} to {}: {}", 
                          abs_path.display(), target_path.display(), e);
            });
        }
    }
    
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

    // Process the file content and write to appropriate files
    let mut in_cargo_field = false;
    for line in content.lines() {
        if line == "---" && in_cargo_field {
            in_cargo_field = false;
        } else if (line == "---" || line == "---cargo") && !in_cargo_field {
            in_cargo_field = true;
        } else if in_cargo_field && line != "---cargo" {
            writeln!(cargo_file, "{}", line).unwrap();
        } else if !in_cargo_field && line != "---" {
            writeln!(rs_file, "{}", line).unwrap();
        }
    }

    drop(rs_file);
    drop(cargo_file);
}

fn extract_relative_paths(content: &str, base_dir: &Path) -> HashSet<(PathBuf, PathBuf)> {
    let mut paths = HashSet::new();
    let mut is_cargo_field = false;
    
    // Define regular expressions for path dependencies
    let single_line_path_re = Regex::new(r#"path\s*=\s*"([^"]+)""#).unwrap();
    let inline_table_re = Regex::new(r#"\{\s*path\s*=\s*"([^"]+)".*\}"#).unwrap();
    
    for line in content.lines() {
        if line == "---" {
            is_cargo_field = false;
            continue;
        } else if line == "---cargo" || line == "---" {
            is_cargo_field = true;
            continue;
        }
        
        if !is_cargo_field {
            continue;
        }
        
        println!("Analyzing cargo line: '{}'", line);
        
        // Find path dependencies
        for (i, re) in [&single_line_path_re, &inline_table_re].iter().enumerate() {
            if let Some(captures) = re.captures(line) {
                println!("Matched regex {}: {}", i, line);
                if let Some(path_match) = captures.get(1) {
                    let rel_path = PathBuf::from(path_match.as_str());
                    println!("Found path: {}", rel_path.display());
                    if rel_path.is_relative() {
                        let abs_path = base_dir.join(&rel_path);
                        println!("Resolved to: {}", abs_path.display());
                        if abs_path.exists() {
                            paths.insert((rel_path, abs_path));
                        } else {
                            eprintln!("Warning: Dependency path does not exist: {}", abs_path.display());
                        }
                    }
                }
            }
        }
    }
    
    paths
}
