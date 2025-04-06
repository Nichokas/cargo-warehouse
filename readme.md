# cargo-warehouse
### A Rust utility that significantly reduces disk space usage and speeds up Rust builds by creating a shared cache for build artifacts across multiple projects.

## 📥 Installation
```bash
cargo install cargo-warehouse
```

## 🔧 Usage
```bash
cargo warehouse
```
This will:

Create a .cargo-cache directory in your home folder (if it doesn't exist)
Set up the necessary directory structure for caching
Link your project's build directories to this centralized cache

## 💡 How It Works

The tool creates symbolic links from your project's target directory subdirectories to a centralized cache in your home directory. This means:

Build artifacts are shared between projects
Dependencies are compiled once and reused
Disk space is saved by eliminating duplicate builds

## ⚠️ Limitations
Requires admin privileges to create symbolic links



This project is licensed under the MIT License - see the LICENSE file for details.
