use crate::{
    feature::{Meta, Subtest},
    features::Es2019WellFormedJsonStringify,
};
impl Meta for Es2019WellFormedJsonStringify {
    fn name(&self) -> &'static str {
        "Well-formed JSON.stringify"
    }

    fn key(&self) -> &'static str {
        "es2019_well_formed_json_stringify"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2019 misc"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-well-formed-stringify"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/JSON/stringify#Well-formed_JSON.stringify()"
    }

    fn exec(&self) -> &'static str {
        "return JSON.stringify('\\uDF06\\uD834') === \"\\\"\\\\udf06\\\\ud834\\\"\"\n  && JSON.stringify('\\uDEAD') === \"\\\"\\\\udead\\\"\";"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
