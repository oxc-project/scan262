use crate::{
    feature::{Meta, Subtest},
    features::Es6NonStrictFunctionSemantics,
};
impl Meta for Es6NonStrictFunctionSemantics {
    fn name(&self) -> &'static str {
        "non-strict function semantics"
    }

    fn key(&self) -> &'static str {
        "es6_non_strict_function_semantics"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "annex b"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-labelled-function-declarations"
    }

    fn significance(&self) -> &'static str {
        "tiny"
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
                name: "hoisted block-level function declaration",
                exec: "// Note: only available outside of strict mode.\nif (!this) return false;\nvar passed = f() === 1;\nfunction f() { return 1; }\n\npassed &= typeof g === 'undefined';\n{ function g() { return 1; } }\npassed &= g() === 1;\n\npassed &= h() === 2;\n{ function h() { return 1; } }\nfunction h() { return 2; }\npassed &= h() === 1;\n\nreturn passed;",
            },
            Subtest {
                name: "labeled function statements",
                exec: "// Note: only available outside of strict mode.\nif (!this) return false;\n\nlabel: function foo() { return 2; }\nreturn foo() === 2;",
            },
            Subtest {
                name: "function statements in if-statement clauses",
                exec: "// Note: only available outside of strict mode.\nif (!this) return false;\n\nif(true) function foo() { return 2; }\nif(false) {} else function bar() { return 3; }\nif(true) function baz() { return 4; } else {}\nif(false) function qux() { return 5; } else function qux() { return 6; }\nreturn foo() === 2 && bar() === 3 && baz() === 4 && qux() === 6;",
            },
        ]
    }
}
