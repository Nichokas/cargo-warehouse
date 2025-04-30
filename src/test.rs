use crate::parse::copy_and_parse;
use std::fs::{self, read_to_string};
use tempfile::TempDir;

#[test]
fn parse_test() {
    // Crear un directorio temporal para las pruebas
    let temp_dir = TempDir::new().unwrap();
    let temp_path = temp_dir.path();
    
    // Crear la estructura de directorios necesaria
    fs::create_dir(temp_path.join("src")).unwrap();
    
    // Crear un archivo de prueba con contenido mixto
    let test_file_path = temp_path.join("test_input.rs");
    let test_content = r#"
---cargo
[dependencies]
comfy-print = "0.3.0"
---

use comfy_print::comfy_println;
fn main() {
    comfy_println!("heyy!!!!");
}
"#;
    fs::write(&test_file_path, test_content).unwrap();
    
    // Ejecutar la función de parsing
    copy_and_parse(test_file_path, temp_path);
    
    // Verificar el contenido de main.rs
    let main_rs_content = read_to_string(temp_path.join("src").join("main.rs")).unwrap();
    assert!(main_rs_content.contains("fn main()"));
    assert!(main_rs_content.contains("comfy_println"));
    assert!(!main_rs_content.contains("---cargo"));
    assert!(!main_rs_content.contains("[dependencies]"));
    
    // Verificar el contenido de Cargo.toml
    let cargo_toml_content = read_to_string(temp_path.join("Cargo.toml")).unwrap();
    assert!(cargo_toml_content.contains("[package]"));
    assert!(cargo_toml_content.contains("name = \"rust_program\""));
    assert!(cargo_toml_content.contains("[dependencies]"));
    assert!(cargo_toml_content.contains("comfy-print = \"0.3.0\""));
}