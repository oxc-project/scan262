use crate::{
    feature::{Meta, Subtest},
    features::Es6NumberProperties,
};
impl Meta for Es6NumberProperties {
    fn name(&self) -> &'static str {
        "Number properties"
    }

    fn key(&self) -> &'static str {
        "es6_number_properties"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-in extensions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-isfinite-number"
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
            Subtest {
                name: "Number.isFinite",
                exec: "return typeof Number.isFinite === 'function';",
            },
            Subtest {
                name: "Number.isInteger",
                exec: "return typeof Number.isInteger === 'function';",
            },
            Subtest {
                name: "Number.isSafeInteger",
                exec: "return typeof Number.isSafeInteger === 'function';",
            },
            Subtest { name: "Number.isNaN", exec: "return typeof Number.isNaN === 'function';" },
            Subtest {
                name: "Number.parseFloat",
                exec: "var actualGlobal = Function('return this')();\nreturn typeof Number.parseFloat === 'function'\n  && Number.parseFloat === actualGlobal.parseFloat;",
            },
            Subtest {
                name: "Number.parseInt",
                exec: "var actualGlobal = Function('return this')();\nreturn typeof Number.parseInt === 'function'\n  && Number.parseInt === actualGlobal.parseInt;",
            },
            Subtest { name: "Number.EPSILON", exec: "return typeof Number.EPSILON === 'number';" },
            Subtest {
                name: "Number.MIN_SAFE_INTEGER",
                exec: "return typeof Number.MIN_SAFE_INTEGER === 'number';",
            },
            Subtest {
                name: "Number.MAX_SAFE_INTEGER",
                exec: "return typeof Number.MAX_SAFE_INTEGER === 'number';",
            },
        ]
    }
}
