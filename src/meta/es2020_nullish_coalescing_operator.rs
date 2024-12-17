use crate::feature::{Meta, Subtest};
use crate::features::Es2020NullishCoalescingOperator;
impl Meta for Es2020NullishCoalescingOperator {
    fn name(&self) -> &'static str {
        "nullish coalescing operator (??)"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-nullish-coalescing"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_operator"
    }
    fn exec(&self) -> &'static str {
        "return (null ?? 42) === 42 &&\n  (undefined ?? 42) === 42 &&\n  (false ?? 42) === false &&\n  ('' ?? 42) === '' &&\n  (0 ?? 42) === 0 &&\n  isNaN(NaN ?? 42);"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
