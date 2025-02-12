use crate::{
    feature::{Meta, Subtest},
    features::Es6StringStaticMethods,
};
impl Meta for Es6StringStaticMethods {
    fn name(&self) -> &'static str {
        "String static methods"
    }

    fn key(&self) -> &'static str {
        "es6_string_static_methods"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-in extensions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-string-constructor"
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name: "String.raw", exec: "return typeof String.raw === 'function';" },
            Subtest {
                name: "String.fromCodePoint",
                exec: "return typeof String.fromCodePoint === 'function';",
            },
        ]
    }
}
