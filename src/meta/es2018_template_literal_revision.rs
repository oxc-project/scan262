use crate::{
    feature::{Meta, Subtest},
    features::Es2018TemplateLiteralRevision,
};
impl Meta for Es2018TemplateLiteralRevision {
    fn name(&self) -> &'static str {
        "template literal revision"
    }

    fn key(&self) -> &'static str {
        "es2018_template_literal_revision"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2018 misc"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-template-literal-revision"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "function tag(strings, a) {\n  return strings[0] === void undefined\n    && strings.raw[0] === \"\\\\01\\\\1\\\\xg\\\\xAg\\\\u0\\\\u0g\\\\u00g\\\\u000g\\\\u{g\\\\u{0\\\\u{110000}\"\n    && strings[1] === \"\\0\"\n    && strings.raw[1] === \"\\\\0\"\n    && a === 0;\n}\nreturn tag`\\01\\1\\xg\\xAg\\u0\\u0g\\u00g\\u000g\\u{g\\u{0\\u{110000}${0}\\0`;"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
