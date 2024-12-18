use crate::feature::{Meta, Subtest};
use crate::features::Es2021PromiseAny;
impl Meta for Es2021PromiseAny {
    fn name(&self) -> &'static str {
        "Promise.any"
    }
    fn key(&self) -> &'static str {
        "es2021_promise_any"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2021 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-promise-any"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/any"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "fulfillment", exec :
            "Promise.any([\n  Promise.reject(1),\n  Promise.resolve(2),\n  Promise.resolve(3)\n]).then(it => {\n  if (it === 2) asyncTestPassed();\n});",
            }, Subtest { name : "AggregateError", exec :
            "Promise.any([\n  Promise.reject(1),\n  Promise.reject(2),\n  Promise.reject(3)\n]).catch (error => {\n  if (error instanceof AggregateError && error.errors.length === 3) asyncTestPassed();\n});",
            },
        ]
    }
}
