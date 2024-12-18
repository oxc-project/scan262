use crate::feature::{Meta, Subtest};
use crate::features::Es2018RegExpNamedCaptureGroups;
impl Meta for Es2018RegExpNamedCaptureGroups {
    fn name(&self) -> &'static str {
        "RegExp named capture groups"
    }
    fn key(&self) -> &'static str {
        "es2018_reg_exp_named_capture_groups"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-named-groups"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions/Groups_and_Ranges"
    }
    fn exec(&self) -> &'static str {
        "var result = /(?<year>\\d{4})-(?<month>\\d{2})-(?<day>\\d{2})/.exec('2016-03-11');\nreturn result.groups.year === '2016'\n  && result.groups.month === '03'\n  && result.groups.day === '11'\n  && result[0] === '2016-03-11'\n  && result[1] === '2016'\n  && result[2] === '03'\n  && result[3] === '11';"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
