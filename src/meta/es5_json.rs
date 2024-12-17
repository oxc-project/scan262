use crate::feature::{Meta, Subtest};
use crate::features::Es5Json;
impl Meta for Es5Json {
    fn name(&self) -> &'static str {
        "JSON"
    }
    fn target(&self) -> &'static str {
        "es5"
    }
    fn category(&self) -> &'static str {
        ""
    }
    fn spec(&self) -> &'static str {
        ""
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON"
    }
    fn exec(&self) -> &'static str {
        "function () {\nreturn typeof JSON === 'object';\n  }"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
