use std::{path::PathBuf, sync::Arc};

use oxc::{
    allocator::Allocator,
    diagnostics::{Error, NamedSource, OxcDiagnostic},
    parser::Parser,
    semantic::SemanticBuilder,
    span::SourceType,
};

use crate::features::RULES;

pub struct Scanner {
    source_path: PathBuf,
    source_text: Arc<str>,
}

impl Scanner {
    pub fn new(source_path: PathBuf, source_text: String) -> Self {
        Self { source_path, source_text: Arc::from(source_text) }
    }

    pub fn scan(&self) -> Vec<Error> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(&self.source_path).unwrap();
        let ret = Parser::new(&allocator, &self.source_text, source_type).parse();

        if !ret.errors.is_empty() {
            return vec![];
        }

        let semantic_ret = SemanticBuilder::new().build(&ret.program);
        let mut diagnostics: Vec<OxcDiagnostic> = vec![];

        for node in semantic_ret.semantic.nodes() {
            for rule in RULES {
                if let Some(span) = rule.test(node) {
                    let diagnostic = OxcDiagnostic::warn(rule.name()).with_label(span);
                    diagnostics.push(diagnostic);
                }
            }
        }

        diagnostics
            .into_iter()
            .map(|d| {
                d.with_source_code(NamedSource::new(
                    self.source_path.to_string_lossy(),
                    Arc::clone(&self.source_text),
                ))
            })
            .collect::<Vec<_>>()
    }
}
