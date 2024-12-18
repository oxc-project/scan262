use crate::feature::{Meta, Subtest};
use crate::features::Es2018SDotAllFlagForRegularExpressions;
impl Meta for Es2018SDotAllFlagForRegularExpressions {
    fn name(&self) -> &'static str {
        "s (dotAll) flag for regular expressions"
    }
    fn key(&self) -> &'static str {
        "es2018_s_dot_all_flag_for_regular_expressions"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://tc39.github.io/ecma262/#sec-get-regexp.prototype.dotAll"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/dotAll"
    }
    fn exec(&self) -> &'static str {
        "const regex = /foo.bar/s;\nreturn regex.test('foo\\nbar');"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
