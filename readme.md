# cargo-warehouse

# ⚠️ WARNING

## This project has been **migrated to Codeberg**

This repository is no longer maintained here.
All development, issues, and updates have moved to:

👉 **https://codeberg.org/nichokas/cargo-warehouse.git**

---

### ❗ Important

* This repository is **archived / read-only**
* No new changes will be pushed here
* Please update your remotes:

```bash
git remote set-url origin https://codeberg.org/nichokas/cargo-warehouse.git
```

### A Rust utility that significantly reduces disk space usage and speeds up Rust builds by creating a shared cache for build artifacts across multiple projects.

## Installation
```bash
cargo install cargo-warehouse
```
## Usage

### Shared Cache Mode
```bash
cargo warehouse
```
This will:

- Create a .cargo-cache directory in your home folder (if it doesn't exist)
- Set up the necessary directory structure for caching
- Link your project's build directories to this centralized cache

### Single File Mode
```bash
cargo warehouse -f path/to/file.rs
```
This mode allows you to run a single Rust file without setting up a full project structure:

- Compiles and runs a standalone `.rs` file
- Supports defining dependencies within the file itself
- Perfect for quick scripts and small programs

#### Example single file format:
```rust
---cargo
[dependencies]
comfy-print = "0.3.0"
---

use comfy_print::comfy_println;
fn main() {
comfy_println!("heyy!!!!");
}
```

## Limitations
On some cases (like using windows) requires admin privileges to create symbolic links; on unix-based systems its not needed unless you dont have rw permissions.

This project is licensed under the MIT License - see the LICENSE file for details.
