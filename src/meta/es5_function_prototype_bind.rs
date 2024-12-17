use crate::feature::{Meta, Subtest};
use crate::features::Es5FunctionPrototypeBind;
impl Meta for Es5FunctionPrototypeBind {
    fn name(&self) -> &'static str {
        "Function.prototype.bind"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/bind"
    }
    fn exec(&self) -> &'static str {
        "function () {\nreturn typeof Function.prototype.bind === 'function';\n  }"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
