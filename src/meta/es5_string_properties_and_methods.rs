use crate::feature::{Meta, Subtest};
use crate::features::Es5StringPropertiesAndMethods;
impl Meta for Es5StringPropertiesAndMethods {
    fn name(&self) -> &'static str {
        "String properties and methods"
    }
    fn key(&self) -> &'static str {
        "es5_string_properties_and_methods"
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
            Subtest { name : "Property access on strings", exec :
            "function () {\nreturn \"foobar\"[3] === \"b\";\n    }", }, Subtest { name :
            "String.prototype.split", exec :
            "function () {\n// all of these tests reflect bugs that es5-shim patches\nreturn typeof String.prototype.split === 'function'\n  && ''.split().length === 1 && ''.split()[0] === ''\n  && ''.split(undefined).length === 1 && ''.split(undefined)[0] === ''\n  && 'ab'.split().length === 1 && 'ab'.split()[0] === 'ab'\n  && 'ab'.split(undefined).length === 1 && 'ab'.split(undefined)[0] === 'ab'\n  && '0'.split(undefined, 0).length === 0\n  && 'ab'.split(/(?:ab)*/).length === 2\n  && '.'.split(/(.?)(.?)/).length === 4\n  && 'tesst'.split(/(s)*/)[1] !== 't'\n  && 'test'.split(/(?:)/, -1).length === 4\n  && ''.split(/.?/).length === 0\n  && '.'.split(/()()/).length === 1;\n    }",
            }, Subtest { name : "String.prototype.substr", exec :
            "function () {\nreturn '0b'.substr(-1) === 'b';\n    }", }, Subtest { name :
            "String.prototype.trim", exec :
            "function () {\nreturn typeof String.prototype.trim === 'function';\n    }",
            },
        ]
    }
}
