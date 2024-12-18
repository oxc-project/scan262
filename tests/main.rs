use std::{fmt::Write, path::PathBuf};

use oxc::diagnostics::Error;
use scan262::{Scanner, FEATURES};

#[test]
fn test() {
    let path = PathBuf::from("test.js");

    let mut features = FEATURES.to_vec();
    features.sort_unstable_by_key(|f| f.key());

    let mut diagnostics = vec![];
    for feature in features {
        let mut passed = true;
        if !feature.spec().is_empty() {
            let scanner = Scanner::new(path.clone(), feature.exec().to_string());
            let d = scanner.scan(&[feature]).diagnostics.1;
            if d.is_empty() {
                passed = false;
            }
            diagnostics.extend(d);
        }
        for subtest in feature.subtests() {
            let scanner = Scanner::new(path.clone(), subtest.exec.to_string());
            let d = scanner.scan(&[feature]).diagnostics.1;
            if d.is_empty() {
                passed = false;
            }
            diagnostics.extend(d);
        }
        if !passed {
            diagnostics.push(Error::msg(format!("Failed: {}", feature.name())));
        }
    }

    let snapshot = diagnostics.into_iter().fold(String::new(), |mut w, d| {
        writeln!(w, "{d:?}").unwrap();
        w
    });

    insta::with_settings!({ prepend_module_to_snapshot => false, snapshot_suffix => "", omit_expression => true }, {
        insta::assert_snapshot!("snapshot", snapshot);
    });
}
