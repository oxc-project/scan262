use crate::feature::{Meta, Subtest};
use crate::features::Es6ProperTailCallsTailCallOptimisation;
impl Meta for Es6ProperTailCallsTailCallOptimisation {
    fn name(&self) -> &'static str {
        "proper tail calls (tail call optimisation)"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "optimisation"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-tail-position-calls"
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
            Subtest { name : "direct recursion", exec :
            "\"use strict\";\nreturn (function f(n){\n  if (n <= 0) {\n    return  \"foo\";\n  }\n  return f(n - 1);\n}(1e6)) === \"foo\";",
            }, Subtest { name : "mutual recursion", exec :
            "\"use strict\";\nfunction f(n){\n  if (n <= 0) {\n    return  \"foo\";\n  }\n  return g(n - 1);\n}\nfunction g(n){\n  if (n <= 0) {\n    return  \"bar\";\n  }\n  return f(n - 1);\n}\nreturn f(1e6) === \"foo\" && f(1e6+1) === \"bar\";",
            },
        ]
    }
}
