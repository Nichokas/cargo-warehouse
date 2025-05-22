---
[dependencies]
mylib = { path = "./mylib" }
---

fn main() {
    println!("{}", mylib::greet());
}
