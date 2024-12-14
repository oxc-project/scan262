#![allow(clippy::print_stdout)]

use std::path::PathBuf;

use scan262::Scanner;

fn main() {
    let source_path = PathBuf::from("test.js");
    let source_text = String::from("x ** y");
    let diagnostics = Scanner::new(source_path, source_text).scan();
    for diagnostic in diagnostics {
        println!("{diagnostic:?}");
    }
}
