use crate::feature::{Meta, Subtest};
use crate::features::Es2022ClassStaticInitializationBlocks;
impl Meta for Es2022ClassStaticInitializationBlocks {
    fn name(&self) -> &'static str {
        "Class static initialization blocks"
    }
    fn key(&self) -> &'static str {
        "es2022_class_static_initialization_blocks"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-class-static-block"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/Static_initialization_blocks"
    }
    fn exec(&self) -> &'static str {
        "let ok = false;\nclass A {\n  static { ok = true; }\n}\nreturn ok;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
