use crate::{
    feature::{Meta, Subtest},
    features::Es6ObjectPrototypeProto,
};
impl Meta for Es6ObjectPrototypeProto {
    fn name(&self) -> &'static str {
        "Object.prototype.__proto__"
    }

    fn key(&self) -> &'static str {
        "es6_object_prototype_proto"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "annex b"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-object.prototype.__proto__"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/proto"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "get prototype",
                exec: "var A = function(){};\nreturn (new A()).__proto__ === A.prototype;",
            },
            Subtest {
                name: "set prototype",
                exec: "var o = {};\no.__proto__ = Array.prototype;\nreturn o instanceof Array;",
            },
            Subtest {
                name: "absent from Object.create(null)",
                exec: "var o = Object.create(null), p = {};\no.__proto__ = p;\nreturn Object.getPrototypeOf(o) !== p;",
            },
            Subtest {
                name: "present in hasOwnProperty()",
                exec: "return Object.prototype.hasOwnProperty('__proto__');",
            },
            Subtest {
                name: "correct property descriptor",
                exec: "var desc = Object.getOwnPropertyDescriptor(Object.prototype,\"__proto__\");\nvar A = function(){};\n\nreturn (desc\n  && \"get\" in desc\n  && \"set\" in desc\n  && desc.configurable\n  && !desc.enumerable);",
            },
            Subtest {
                name: "present in Object.getOwnPropertyNames()",
                exec: "return Object.getOwnPropertyNames(Object.prototype).indexOf('__proto__') > -1;",
            },
        ]
    }
}
