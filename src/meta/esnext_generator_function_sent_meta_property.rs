use crate::{
    feature::{Meta, Subtest},
    features::EsnextGeneratorFunctionSentMetaProperty,
};
impl Meta for EsnextGeneratorFunctionSentMetaProperty {
    fn name(&self) -> &'static str {
        "Generator function.sent Meta Property"
    }

    fn key(&self) -> &'static str {
        "esnext_generator_function_sent_meta_property"
    }

    fn target(&self) -> &'static str {
        "esnext"
    }

    fn category(&self) -> &'static str {
        "Stage 2"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-function.sent"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "var result;\nfunction* generator() {\n  result = function.sent;\n}\nvar iter = generator();\niter.next('tromple');\nreturn result === 'tromple';"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
