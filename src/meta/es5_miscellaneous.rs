use crate::feature::{Meta, Subtest};
use crate::features::Es5Miscellaneous;
impl Meta for Es5Miscellaneous {
    fn name(&self) -> &'static str {
        "Miscellaneous"
    }
    fn key(&self) -> &'static str {
        "es5_miscellaneous"
    }
    fn target(&self) -> &'static str {
        "es5"
    }
    fn category(&self) -> &'static str {
        ""
    }
    fn spec(&self) -> &'static str {
        ""
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
            Subtest { name : "Function.prototype.apply permits array-likes", exec :
            "function () {\ntry {\n  return (function(a,b) { return a === 1 && b === 2; }).apply({}, {0:1, 1:2, length:2});\n} catch (e) {\n  return false;\n}\n    }",
            }, Subtest { name : "parseInt ignores leading zeros", exec :
            "function () {\nreturn parseInt('010') === 10;\n    }", }, Subtest { name :
            "Function \"prototype\" property is non-enumerable", exec :
            "return !Function().propertyIsEnumerable('prototype');", }, Subtest { name :
            "Arguments toStringTag is \"Arguments\"", exec :
            "return (function(){ return Object.prototype.toString.call(arguments) === '[object Arguments]'; }());",
            }, Subtest { name : "Zero-width chars in identifiers", exec :
            "var _\\u200c\\u200d = true;\nreturn _\\u200c\\u200d;", }, Subtest { name :
            "Unreserved words", exec :
            "var abstract, boolean, byte, char, double, final, float, goto, int, long,\n  native, short, synchronized, transient, volatile;\nreturn true;",
            }, Subtest { name :
            "Enumerable properties can be shadowed by non-enumerables", exec :
            "var result = true;\nObject.prototype.length = 42;\nfor (var i in Function) {\n    if (i === 'length') {\n        result = false;\n    }\n}\ndelete Object.prototype.length;\nreturn result;",
            }, Subtest { name : "Thrown functions have proper \"this\" values", exec :
            "try {\n  throw function() { return !('a' in this); };\n}\ncatch(e) {\n  var a = true;\n  return e();\n}",
            },
        ]
    }
}
