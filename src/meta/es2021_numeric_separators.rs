use crate::feature::{Meta, Subtest};
use crate::features::Es2021NumericSeparators;
impl Meta for Es2021NumericSeparators {
    fn name(&self) -> &'static str {
        "numeric separators"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2021 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-numeric-separator"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "return 1_000_000.000_001 === 1000000.000001 &&\n  0b1010_0001_1000_0101 === 0b1010000110000101;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
