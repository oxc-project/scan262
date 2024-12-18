use crate::feature::{Meta, Subtest};
use crate::features::Es2020OptionalChainingOperator;
impl Meta for Es2020OptionalChainingOperator {
    fn name(&self) -> &'static str {
        "optional chaining operator (?.)"
    }
    fn key(&self) -> &'static str {
        "es2020_optional_chaining_operator"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-optional-chaining"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Optional_chaining"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "optional property access", exec :
            "var foo = { baz: 42 };\nvar bar = null;\nreturn foo?.baz === 42 && bar?.baz === void undefined;",
            }, Subtest { name : "optional bracket access", exec :
            "var foo = { baz: 42 };\nvar bar = null;\nreturn foo?.['baz'] === 42 && bar?.['baz'] === void undefined;",
            }, Subtest { name : "optional method call", exec :
            "var foo = { baz: function () { return this.value; }, value: 42 };\nvar bar = null;\nreturn foo?.baz() === 42 && bar?.baz() === void undefined;",
            }, Subtest { name : "optional function call", exec :
            "var foo = { baz: function () { return 42; } };\nvar bar = {};\nfunction baz() { return 42; };\nvar n;\nreturn foo.baz?.() === 42 && bar.baz?.() === void undefined && baz?.() === 42 && n?.() === void undefined;",
            }, Subtest { name : "spread parameters after optional chaining", exec :
            "var fn = null;\nvar n = null;\nvar o = {};\n\nreturn fn?.(...[], 1) === void undefined && fn?.(...[], ...[]) === void undefined && o.method?.(...[], 1) === void undefined && n?.method(...[], 1) === void undefined;",
            },
        ]
    }
}
