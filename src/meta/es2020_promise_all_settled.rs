use crate::feature::{Meta, Subtest};
use crate::features::Es2020PromiseAllSettled;
impl Meta for Es2020PromiseAllSettled {
    fn name(&self) -> &'static str {
        "Promise.allSettled"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-promise-allSettled"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/allSettled"
    }
    fn exec(&self) -> &'static str {
        "Promise.allSettled([\n  Promise.resolve(1),\n  Promise.reject(2),\n  Promise.resolve(3)\n]).then(it => {\n  if (\n    it.length === 3 &&\n    it[0].status === 'fulfilled' && it[0].value === 1 &&\n    it[1].status === 'rejected' && it[1].reason === 2 &&\n    it[2].status === 'fulfilled' && it[2].value === 3\n  ) asyncTestPassed();\n});"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
