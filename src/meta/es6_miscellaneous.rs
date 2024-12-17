use crate::feature::{Meta, Subtest};
use crate::features::Es6Miscellaneous;
impl Meta for Es6Miscellaneous {
    fn name(&self) -> &'static str {
        "miscellaneous"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-additions-and-changes-that-introduce-incompatibilities-with-prior-editions"
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
            Subtest { name : "duplicate property names in strict mode", exec :
            "'use strict';\nreturn this === void undefined && ({ a:1, a:1 }).a === 1;",
            }, Subtest { name : "no semicolon needed after do-while", exec :
            "do {} while (false) return true;", }, Subtest { name :
            "no assignments allowed in for-in head in strict mode", exec :
            "'use strict';\ntry {\n  eval('for (var i = 0 in {}) {}');\n}\ncatch(e) {\n  return true;\n}",
            }, Subtest { name : "accessors aren't constructors", exec :
            "var f = (Object.getOwnPropertyDescriptor({get a(){}}, 'a')).get;\ntry {\n  new f;\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "Invalid Date", exec :
            "return new Date(NaN) + \"\" === \"Invalid Date\";", }, Subtest { name :
            "RegExp constructor can alter flags", exec :
            "return new RegExp(/./im, \"g\").global === true;", }, Subtest { name :
            "RegExp.prototype.toString generic and uses \"flags\" property", exec :
            "return RegExp.prototype.toString.call({source: 'foo', flags: 'bar'}) === '/foo/bar';",
            }, Subtest { name : "built-in prototypes are not instances", exec :
            "try {\n  RegExp.prototype.exec(); return false;\n} catch(e) {}\ntry {\n  Date.prototype.valueOf(); return false;\n} catch(e) {}\n\nif (![Error, EvalError, RangeError, ReferenceError, SyntaxError, TypeError, URIError].every(function (E) {\n    return Object.prototype.toString.call(E.prototype) === '[object Object]';\n})) {\n  return false;\n}\n\nreturn true;",
            }, Subtest { name : "function 'length' is configurable", exec :
            "var fn = function(a, b) {};\n\nvar desc = Object.getOwnPropertyDescriptor(fn, \"length\");\nif (desc.configurable) {\n  Object.defineProperty(fn, \"length\", { value: 1 });\n  return fn.length === 1;\n}\n\nreturn false;",
            },
        ]
    }
}
