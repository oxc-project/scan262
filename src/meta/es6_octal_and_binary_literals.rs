use crate::{
    feature::{Meta, Subtest},
    features::Es6OctalAndBinaryLiterals,
};
impl Meta for Es6OctalAndBinaryLiterals {
    fn name(&self) -> &'static str {
        "octal and binary literals"
    }

    fn key(&self) -> &'static str {
        "es6_octal_and_binary_literals"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "syntax"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-literals-numeric-literals"
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
            Subtest { name: "octal literals", exec: "return 0o10 === 8 && 0O10 === 8;" },
            Subtest { name: "binary literals", exec: "return 0b10 === 2 && 0B10 === 2;" },
            Subtest { name: "octal supported by Number()", exec: "return Number('0o1') === 1;" },
            Subtest { name: "binary supported by Number()", exec: "return Number('0b1') === 1;" },
        ]
    }
}
