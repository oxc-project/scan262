use crate::{
    feature::{Meta, Subtest},
    features::Es2019ObjectFromEntries,
};
impl Meta for Es2019ObjectFromEntries {
    fn name(&self) -> &'static str {
        "Object.fromEntries"
    }

    fn key(&self) -> &'static str {
        "es2019_object_from_entries"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2019 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-object-from-entries"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/fromEntries"
    }

    fn exec(&self) -> &'static str {
        "var object = Object.fromEntries(new Map([['foo', 42], ['bar', 23]]));\nreturn object.foo === 42 && object.bar === 23;"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
