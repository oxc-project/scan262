use crate::feature::{Meta, Subtest};
use crate::features::Es6RestParameters;
impl Meta for Es6RestParameters {
    fn name(&self) -> &'static str {
        "rest parameters"
    }
    fn key(&self) -> &'static str {
        "es6_rest_parameters"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-function-definitions"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/rest_parameters"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "return (function (foo, ...args) {\n  return args instanceof Array && args + \"\" === \"bar,baz\";\n}(\"foo\", \"bar\", \"baz\"));",
            }, Subtest { name : "function 'length' property", exec :
            "return function(a, ...b){}.length === 1 && function(...c){}.length === 0;",
            }, Subtest { name : "arguments object interaction", exec :
            "return (function (foo, ...args) {\n  foo = \"qux\";\n  // The arguments object is not mapped to the\n  // parameters, even outside of strict mode.\n  return arguments.length === 3\n    && arguments[0] === \"foo\"\n    && arguments[1] === \"bar\"\n    && arguments[2] === \"baz\";\n}(\"foo\", \"bar\", \"baz\"));",
            }, Subtest { name : "can't be used in setters", exec :
            "return (function (...args) {\n  try {\n    eval(\"({set e(...args){}})\");\n  } catch(e) {\n    return true;\n  }\n}());",
            }, Subtest { name : "new Function() support", exec :
            "return new Function(\"a\", \"...b\",\n  \"return b instanceof Array && a+b === 'foobar,baz';\"\n)('foo','bar','baz');",
            },
        ]
    }
}
