use crate::feature::{Meta, Subtest};
use crate::features::Es2017TrailingCommasInFunctionSyntax;
impl Meta for Es2017TrailingCommasInFunctionSyntax {
    fn name(&self) -> &'static str {
        "trailing commas in function syntax"
    }
    fn key(&self) -> &'static str {
        "es2017_trailing_commas_in_function_syntax"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-trailing-function-commas"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "in parameter lists", exec :
            "return typeof function f( a, b, ) {} === 'function';" }, Subtest { name :
            "in argument lists", exec : "return Math.min(1,2,3,) === 1;" },
        ]
    }
}
