use crate::feature::{Meta, Subtest};
use crate::features::Es2025PromiseTry;
impl Meta for Es2025PromiseTry {
    fn name(&self) -> &'static str {
        "Promise.try"
    }
    fn key(&self) -> &'static str {
        "es2025_promise_try"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2025 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-promise-try"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/try"
    }
    fn exec(&self) -> &'static str {
        "var called = false;\nvar argsMatch = false;\nvar p = Promise.try(function () { called = true; })\nvar p2 = Promise.try(function () {\n  'use strict';\n  argsMatch = this === undefined && arguments.length === 2 && args[0] === p && args[1] === 2;\n}, [p, 2]);\n\nreturn p instanceof Promise && called && argsMatch;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
