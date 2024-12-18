#![allow(clippy::print_stdout)]

use std::{
    fs,
    path::{Path, PathBuf},
};

use oxc::{diagnostics::DiagnosticService, span::VALID_EXTENSIONS};
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use walkdir::WalkDir;

use scan262::{command, Scanner, FEATURES};

fn get_paths(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext.to_string_lossy().as_ref()))
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>()
}

fn main() {
    let command = command().run();

    let paths: Vec<PathBuf> = if command.paths.is_empty() {
        get_paths(Path::new("."))
    } else {
        command.paths.iter().flat_map(|p| get_paths(p)).collect()
    };

    let mut diagnostic_service = DiagnosticService::default();

    rayon::spawn({
        let tx_error = diagnostic_service.sender().clone();
        move || {
            paths.par_iter().for_each(|path| {
                let source_text = fs::read_to_string(path).unwrap();
                let scanner = Scanner::new(path.to_path_buf(), source_text);
                let ret = scanner.scan(FEATURES);
                tx_error.send(Some(ret.diagnostics)).unwrap();
            });
            tx_error.send(None).unwrap();
        }
    });

    diagnostic_service.run();
}
