use crate::feature::{Meta, Subtest};
use crate::features::Es5ObjectStaticMethods;
impl Meta for Es5ObjectStaticMethods {
    fn name(&self) -> &'static str {
        "Object static methods"
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
        "large"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Object.create", exec :
            "function () {\nreturn typeof Object.create === 'function';\n    }", },
            Subtest { name : "Object.defineProperty", exec :
            "function () {\nreturn typeof Object.defineProperty === 'function';\n    }",
            }, Subtest { name : "Object.defineProperties", exec :
            "function () {\nreturn typeof Object.defineProperties === 'function';\n    }",
            }, Subtest { name : "Object.getPrototypeOf", exec :
            "function () {\nreturn typeof Object.getPrototypeOf === 'function';\n    }",
            }, Subtest { name : "Object.keys", exec :
            "function () {\nreturn typeof Object.keys === 'function';\n    }", }, Subtest
            { name : "Object.seal", exec :
            "function () {\nreturn typeof Object.seal === 'function';\n    }", }, Subtest
            { name : "Object.freeze", exec :
            "function () {\nreturn typeof Object.freeze === 'function';\n    }", },
            Subtest { name : "Object.preventExtensions", exec :
            "function () {\nreturn typeof Object.preventExtensions === 'function';\n    }",
            }, Subtest { name : "Object.isSealed", exec :
            "function () {\nreturn typeof Object.isSealed === 'function';\n    }", },
            Subtest { name : "Object.isFrozen", exec :
            "function () {\nreturn typeof Object.isFrozen === 'function';\n    }", },
            Subtest { name : "Object.isExtensible", exec :
            "function () {\nreturn typeof Object.isExtensible === 'function';\n    }", },
            Subtest { name : "Object.getOwnPropertyDescriptor", exec :
            "function () {\nreturn typeof Object.getOwnPropertyDescriptor === 'function';\n    }",
            }, Subtest { name : "Object.getOwnPropertyNames", exec :
            "function () {\nreturn typeof Object.getOwnPropertyNames === 'function';\n    }",
            },
        ]
    }
}
