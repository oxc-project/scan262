use oxc::{diagnostics::OxcDiagnostic, span::Span};

use crate::feature::Feature;

#[derive(Default)]
pub struct Ctx {
    pub diagnostics: Vec<OxcDiagnostic>,
}

impl Ctx {
    pub fn diagnostic(&mut self, feature: &dyn Feature, span: Span) {
        self.diagnostics.push(
            OxcDiagnostic::warn(feature.name())
                .with_error_code_scope(feature.target())
                .with_label(span)
                .with_url(feature.mdn()),
        );
    }

    pub fn diagnostics(self) -> Vec<OxcDiagnostic> {
        self.diagnostics
    }
}
