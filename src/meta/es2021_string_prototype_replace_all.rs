use crate::feature::{Meta, Subtest};
use crate::features::Es2021StringPrototypeReplaceAll;
impl Meta for Es2021StringPrototypeReplaceAll {
    fn name(&self) -> &'static str {
        "String.prototype.replaceAll"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2021 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-string-replace-all"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/replaceAll"
    }
    fn exec(&self) -> &'static str {
        "return 'q=query+string+parameters'.replaceAll('+', ' ') === 'q=query string parameters';"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
