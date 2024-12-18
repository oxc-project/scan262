#![allow(clippy::print_stdout)]

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use oxc::diagnostics::DiagnosticService;
use rayon::iter::IntoParallelRefIterator;
use rayon::prelude::*;
use walkdir::WalkDir;

use scan262::{command, Scanner, FEATURES};

pub const VALID_EXTENSIONS: [&str; 3] = ["js", "mjs", "cjs"];

fn get_paths(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| VALID_EXTENSIONS.contains(&ext))
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

    let mut diagnostic_service = DiagnosticService::default().with_quiet(command.quiet);
    let paths_len = paths.len();
    let all_stats = Arc::new(Mutex::new(vec![]));

    rayon::spawn({
        let tx_error = diagnostic_service.sender().clone();
        let all_stats = Arc::clone(&all_stats);
        move || {
            let stats = paths
                .par_iter()
                .filter_map(|path| {
                    let Ok(source_text) = fs::read_to_string(path) else { return None };
                    let scanner = Scanner::new(path.to_path_buf(), source_text);
                    let ret = scanner.scan(FEATURES);
                    tx_error.send(Some(ret.diagnostics)).unwrap();
                    Some(ret.stats)
                })
                .reduce(
                    || vec![0; FEATURES.len()],
                    |mut a, b| {
                        for (i, c) in b.into_iter().enumerate() {
                            a[i] += c;
                        }
                        a
                    },
                );
            *all_stats.lock().unwrap() = stats;
            tx_error.send(None).unwrap();
        }
    });

    diagnostic_service.run();

    let all_stats = all_stats.lock().unwrap();
    for (i, feature) in FEATURES.iter().enumerate() {
        let stats = all_stats[i];
        if stats > 0 {
            println!("{}: {}", feature.name(), stats);
        }
    }

    println!("Total number of files scanned: {}", paths_len);
}
