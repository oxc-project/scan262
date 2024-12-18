use crate::{
    feature::{Meta, Subtest},
    features::Es2025DuplicateNamedCapturingGroups,
};
impl Meta for Es2025DuplicateNamedCapturingGroups {
    fn name(&self) -> &'static str {
        "Duplicate named capturing groups"
    }

    fn key(&self) -> &'static str {
        "es2025_duplicate_named_capturing_groups"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2025 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-duplicate-named-capturing-groups"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "return /(?<year>[0-9]{4})-[0-9]{2}|[0-9]{2}-(?<year>[0-9]{4})/.test(\"12-1995\");"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
