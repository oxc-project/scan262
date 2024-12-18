use crate::feature::{Meta, Subtest};
use crate::features::Es2017RegExpUFlagCaseFolding;
impl Meta for Es2017RegExpUFlagCaseFolding {
    fn name(&self) -> &'static str {
        "RegExp \"u\" flag, case folding"
    }
    fn key(&self) -> &'static str {
        "es2017_reg_exp_u_flag_case_folding"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 misc"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/ecma262/pull/525"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "return \"ſ\".match(/\\w/iu) && !\"ſ\".match(/\\W/iu)\n  && \"\\u212a\".match(/\\w/iu) && !\"\\u212a\".match(/\\W/iu)\n  && \"\\u212a\".match(/.\\b/iu) && \"ſ\".match(/.\\b/iu)\n  && !\"\\u212a\".match(/.\\B/iu) && !\"ſ\".match(/.\\B/iu);"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
