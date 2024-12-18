use std::path::PathBuf;

use oxc::{
    allocator::Allocator,
    diagnostics::{DiagnosticService, Error},
    parser::{ParseOptions, Parser},
    semantic::SemanticBuilder,
    span::SourceType,
};

use crate::{ctx::Ctx, Feature};

pub struct Scanner {
    source_path: PathBuf,
    source_text: String,
}

impl Scanner {
    pub fn new(source_path: PathBuf, source_text: String) -> Self {
        Self { source_path, source_text }
    }

    pub fn scan(&self, features: &[&dyn Feature]) -> Option<(PathBuf, Vec<Error>)> {
        let allocator = Allocator::default();
        let source_type = SourceType::from_path(&self.source_path).unwrap();
        let ret = Parser::new(&allocator, &self.source_text, source_type)
            .with_options(ParseOptions {
                allow_return_outside_function: true,
                ..ParseOptions::default()
            })
            .parse();

        if !ret.errors.is_empty() {
            return None;
        }

        let semantic_ret = SemanticBuilder::new().build(&ret.program);
        let mut ctx = Ctx::default();

        for node in semantic_ret.semantic.nodes() {
            for feature in features {
                feature.test(node, &mut ctx);
            }
        }

        Some(DiagnosticService::wrap_diagnostics(
            &self.source_path,
            &self.source_text,
            ctx.diagnostics(),
        ))
    }
}
