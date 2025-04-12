---cargo
[package]
name = "test"
edition = "2024"

[dependencies]
comfy-print = "0.3.0"
---

use comfy_print::comfy_println;
fn main() {
    comfy_println!("heyy!!!!");
}
