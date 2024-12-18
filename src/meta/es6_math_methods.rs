use crate::{
    feature::{Meta, Subtest},
    features::Es6MathMethods,
};
impl Meta for Es6MathMethods {
    fn name(&self) -> &'static str {
        "Math methods"
    }

    fn key(&self) -> &'static str {
        "es6_math_methods"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-in extensions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-math"
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
            Subtest { name: "Math.clz32", exec: "return typeof Math.clz32 === \"function\";" },
            Subtest { name: "Math.imul", exec: "return typeof Math.imul === \"function\";" },
            Subtest { name: "Math.sign", exec: "return typeof Math.sign === \"function\";" },
            Subtest { name: "Math.log10", exec: "return typeof Math.log10 === \"function\";" },
            Subtest { name: "Math.log2", exec: "return typeof Math.log2 === \"function\";" },
            Subtest { name: "Math.log1p", exec: "return typeof Math.log1p === \"function\";" },
            Subtest { name: "Math.expm1", exec: "return typeof Math.expm1 === \"function\";" },
            Subtest { name: "Math.cosh", exec: "return typeof Math.cosh === \"function\";" },
            Subtest { name: "Math.sinh", exec: "return typeof Math.sinh === \"function\";" },
            Subtest { name: "Math.tanh", exec: "return typeof Math.tanh === \"function\";" },
            Subtest { name: "Math.acosh", exec: "return typeof Math.acosh === \"function\";" },
            Subtest { name: "Math.asinh", exec: "return typeof Math.asinh === \"function\";" },
            Subtest { name: "Math.atanh", exec: "return typeof Math.atanh === \"function\";" },
            Subtest { name: "Math.trunc", exec: "return typeof Math.trunc === \"function\";" },
            Subtest { name: "Math.fround", exec: "return typeof Math.fround === \"function\";" },
            Subtest { name: "Math.cbrt", exec: "return typeof Math.cbrt === \"function\";" },
            Subtest {
                name: "Math.hypot",
                exec: "return Math.hypot() === 0 &&\n  Math.hypot(1) === 1 &&\n  Math.hypot(9, 12, 20) === 25 &&\n  Math.hypot(27, 36, 60, 100) === 125;",
            },
        ]
    }
}
