use crate::feature::{Meta, Subtest};
use crate::features::Es2024PromiseWithResolvers;
impl Meta for Es2024PromiseWithResolvers {
    fn name(&self) -> &'static str {
        "Promise.withResolvers"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2024 features"
    }
    fn spec(&self) -> &'static str {
        "https://tc39.es/proposal-promise-with-resolvers/"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/withResolvers"
    }
    fn exec(&self) -> &'static str {
        "var obj = Promise.withResolvers();\nreturn obj instanceof Object\n  && obj.promise instanceof Promise\n  && typeof obj.resolve === 'function'\n  && typeof obj.reject === 'function';"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
