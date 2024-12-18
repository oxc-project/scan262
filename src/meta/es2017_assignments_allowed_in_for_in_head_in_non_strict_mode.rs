use crate::feature::{Meta, Subtest};
use crate::features::Es2017AssignmentsAllowedInForInHeadInNonStrictMode;
impl Meta for Es2017AssignmentsAllowedInForInHeadInNonStrictMode {
    fn name(&self) -> &'static str {
        "assignments allowed in for-in head in non-strict mode"
    }
    fn key(&self) -> &'static str {
        "es2017_assignments_allowed_in_for_in_head_in_non_strict_mode"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 annex b"
    }
    fn spec(&self) -> &'static str {
        "https://tc39.github.io/ecma262/#sec-initializers-in-forin-statement-heads"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...in#Compatibility_Initializer_expressions_in_strict_mode"
    }
    fn exec(&self) -> &'static str {
        "for (var i = 0 in {}) {}\nreturn i === 0;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
