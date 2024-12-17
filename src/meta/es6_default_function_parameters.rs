use crate::feature::{Meta, Subtest};
use crate::features::Es6DefaultFunctionParameters;
impl Meta for Es6DefaultFunctionParameters {
    fn name(&self) -> &'static str {
        "default function parameters"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-functiondeclarationinstantiation"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Default_parameters"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "return (function (a = 1, b = 2) { return a === 3 && b === 2; }(3));", },
            Subtest { name : "explicit undefined defers to the default", exec :
            "return (function (a = 1, b = 2) { return a === 1 && b === 3; }(undefined, 3));",
            }, Subtest { name : "defaults can refer to previous params", exec :
            "return (function (a, b = a) { return b === 5; }(5));", }, Subtest { name :
            "arguments object interaction", exec :
            "return (function (a = \"baz\", b = \"qux\", c = \"quux\") {\n  a = \"corge\";\n  // The arguments object is not mapped to the\n  // parameters, even outside of strict mode.\n  return arguments.length === 2\n    && arguments[0] === \"foo\"\n    && arguments[1] === \"bar\";\n}(\"foo\", \"bar\"));",
            }, Subtest { name : "temporal dead zone", exec :
            "return (function(x = 1) {\n  try {\n    eval(\"(function(a=a){}())\");\n    return false;\n  } catch(e) {}\n  try {\n    eval(\"(function(a=b,b){}())\");\n    return false;\n  } catch(e) {}\n  return true;\n}());",
            }, Subtest { name : "separate scope", exec :
            "return (function(a=function(){\n  return typeof b === 'undefined';\n}){\n  var b = 1;\n  return a();\n}());",
            }, Subtest { name : "new Function() support", exec :
            "return new Function(\"a = 1\", \"b = 2\",\n  \"return a === 3 && b === 2;\"\n)(3);",
            },
        ]
    }
}
