use crate::feature::{Meta, Subtest};
use crate::features::Es6ObjectStaticMethodsAcceptPrimitives;
impl Meta for Es6ObjectStaticMethodsAcceptPrimitives {
    fn name(&self) -> &'static str {
        "Object static methods accept primitives"
    }
    fn key(&self) -> &'static str {
        "es6_object_static_methods_accept_primitives"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-object-constructor"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Object.getPrototypeOf", exec :
            "return Object.getPrototypeOf('a').constructor === String;", }, Subtest {
            name : "Object.getOwnPropertyDescriptor", exec :
            "return Object.getOwnPropertyDescriptor('a', 'foo') === void undefined;", },
            Subtest { name : "Object.getOwnPropertyNames", exec :
            "var s = Object.getOwnPropertyNames('a');\nreturn s.length === 2 &&\n  ((s[0] === 'length' && s[1] === '0') || (s[0] === '0' && s[1] === 'length'));",
            }, Subtest { name : "Object.seal", exec : "return Object.seal('a') === 'a';",
            }, Subtest { name : "Object.freeze", exec :
            "return Object.freeze('a') === 'a';", }, Subtest { name :
            "Object.preventExtensions", exec :
            "return Object.preventExtensions('a') === 'a';", }, Subtest { name :
            "Object.isSealed", exec : "return Object.isSealed('a') === true;", }, Subtest
            { name : "Object.isFrozen", exec : "return Object.isFrozen('a') === true;",
            }, Subtest { name : "Object.isExtensible", exec :
            "return Object.isExtensible('a') === false;", }, Subtest { name :
            "Object.keys", exec :
            "var s = Object.keys('a');\nreturn s.length === 1 && s[0] === '0';", },
        ]
    }
}
