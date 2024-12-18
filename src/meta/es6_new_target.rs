use crate::feature::{Meta, Subtest};
use crate::features::Es6NewTarget;
impl Meta for Es6NewTarget {
    fn name(&self) -> &'static str {
        "new.target"
    }
    fn key(&self) -> &'static str {
        "es6_new_target"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-built-in-function-objects"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/new.target"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "in constructors", exec :
            "var passed = false;\nnew function f() {\n  passed = (new.target === f);\n}();\n(function() {\n  passed &= (new.target === void undefined);\n}());\nreturn passed;",
            }, Subtest { name : "assignment is an early error", exec :
            "var passed = false;\nnew function f() {\n  passed = (new.target === f);\n}();\n\ntry {\n  Function(\"new.target = function(){};\");\n} catch(e) {\n  return passed;\n}",
            },
        ]
    }
}
