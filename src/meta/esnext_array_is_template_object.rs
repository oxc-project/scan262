use crate::{
    feature::{Meta, Subtest},
    features::EsnextArrayIsTemplateObject,
};
impl Meta for EsnextArrayIsTemplateObject {
    fn name(&self) -> &'static str {
        "Array.isTemplateObject"
    }

    fn key(&self) -> &'static str {
        "esnext_array_is_template_object"
    }

    fn target(&self) -> &'static str {
        "esnext"
    }

    fn category(&self) -> &'static str {
        "Stage 2"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-array-is-template-object"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "return !Array.isTemplateObject([])\n  && Array.isTemplateObject((it => it)`a${1}c`);"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
