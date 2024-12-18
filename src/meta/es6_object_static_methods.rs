use crate::feature::{Meta, Subtest};
use crate::features::Es6ObjectStaticMethods;
impl Meta for Es6ObjectStaticMethods {
    fn name(&self) -> &'static str {
        "Object static methods"
    }
    fn key(&self) -> &'static str {
        "es6_object_static_methods"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-in extensions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-object-constructor"
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
            Subtest { name : "Object.assign", exec :
            "var o = Object.assign({a:true}, {b:true}, {c:true});\nreturn \"a\" in o && \"b\" in o && \"c\" in o;",
            }, Subtest { name : "Object.is", exec :
            "return typeof Object.is === 'function' &&\n  Object.is(NaN, NaN) &&\n !Object.is(-0, 0);",
            }, Subtest { name : "Object.getOwnPropertySymbols", exec :
            "var o = {};\nvar sym = Symbol(), sym2 = Symbol(), sym3 = Symbol();\no[sym]  = true;\no[sym2] = true;\no[sym3] = true;\nvar result = Object.getOwnPropertySymbols(o);\nreturn result[0] === sym\n  && result[1] === sym2\n  && result[2] === sym3;",
            }, Subtest { name : "Object.setPrototypeOf", exec :
            "return Object.setPrototypeOf({}, Array.prototype) instanceof Array;", },
        ]
    }
}
