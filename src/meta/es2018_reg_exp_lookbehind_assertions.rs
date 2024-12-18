use crate::feature::{Meta, Subtest};
use crate::features::Es2018RegExpLookbehindAssertions;
impl Meta for Es2018RegExpLookbehindAssertions {
    fn name(&self) -> &'static str {
        "RegExp Lookbehind Assertions"
    }
    fn key(&self) -> &'static str {
        "es2018_reg_exp_lookbehind_assertions"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-lookbehind"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions/Assertions"
    }
    fn exec(&self) -> &'static str {
        "return /(?<=a)b/.test('ab') && /(?<!a)b/.test('cb') &&\n       !/(?<=a)b/.test('b');"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
