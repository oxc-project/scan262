#![allow(clippy::print_stdout)]

use std::path::PathBuf;

use oxc::diagnostics::DiagnosticService;
use scan262::{Scanner, FEATURES};

fn main() {
    let source_path = PathBuf::from("test.js");
    let source_text = String::from("x ** y");

    let mut diagnostic_service = DiagnosticService::default();

    rayon::spawn({
        let tx_error = diagnostic_service.sender().clone();
        let scanner = Scanner::new(source_path, source_text);
        move || {
            tx_error.send(scanner.scan(FEATURES)).unwrap();
            tx_error.send(None).unwrap();
        }
    });

    diagnostic_service.run();
}
