use crate::feature::{Meta, Subtest};
use crate::features::Es2017StringPadding;
impl Meta for Es2017StringPadding {
    fn name(&self) -> &'static str {
        "String padding"
    }
    fn key(&self) -> &'static str {
        "es2017_string_padding"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-string-pad-start-end"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "String.prototype.padStart", exec :
            "return 'hello'.padStart(10) === '     hello'\n  && 'hello'.padStart(10, '1234') === '12341hello'\n  && 'hello'.padStart() === 'hello'\n  && 'hello'.padStart(6, '123') === '1hello'\n  && 'hello'.padStart(3) === 'hello'\n  && 'hello'.padStart(3, '123') === 'hello';"
            }, Subtest { name : "String.prototype.padEnd", exec :
            "return 'hello'.padEnd(10) === 'hello     '\n  && 'hello'.padEnd(10, '1234') === 'hello12341'\n  && 'hello'.padEnd() === 'hello'\n  && 'hello'.padEnd(6, '123') === 'hello1'\n  && 'hello'.padEnd(3) === 'hello'\n  && 'hello'.padEnd(3, '123') === 'hello';"
            },
        ]
    }
}
