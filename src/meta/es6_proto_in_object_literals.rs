use crate::feature::{Meta, Subtest};
use crate::features::Es6ProtoInObjectLiterals;
impl Meta for Es6ProtoInObjectLiterals {
    fn name(&self) -> &'static str {
        "__proto__ in object literals"
    }
    fn key(&self) -> &'static str {
        "es6_proto_in_object_literals"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "annex b"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-__proto__-property-names-in-object-initializers"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto#Specifications"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic support", exec :
            "return { __proto__ : [] } instanceof Array\n  && !({ __proto__ : null } instanceof Object);"
            }, Subtest { name : "multiple __proto__ is an error", exec :
            "try {\n  eval(\"({ __proto__ : [], __proto__: {} })\");\n}\ncatch(e) {\n  return true;\n}"
            }, Subtest { name : "not a computed property", exec :
            "if (!({ __proto__ : [] } instanceof Array)) {\n  return false;\n}\nvar a = \"__proto__\";\nreturn !({ [a] : [] } instanceof Array);"
            }, Subtest { name : "not a shorthand property", exec :
            "if (!({ __proto__ : [] } instanceof Array)) {\n  return false;\n}\nvar __proto__ = [];\nreturn !({ __proto__ } instanceof Array);"
            }, Subtest { name : "not a shorthand method", exec :
            "if (!({ __proto__ : [] } instanceof Array)) {\n  return false;\n}\nreturn !({ __proto__(){} } instanceof Function);"
            },
        ]
    }
}
