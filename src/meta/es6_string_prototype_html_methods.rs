use crate::feature::{Meta, Subtest};
use crate::features::Es6StringPrototypeHtmlMethods;
impl Meta for Es6StringPrototypeHtmlMethods {
    fn name(&self) -> &'static str {
        "String.prototype HTML methods"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "annex b"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-string.prototype.anchor"
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
            Subtest { name : "existence", exec :
            "var i, names = [\"anchor\", \"big\", \"bold\", \"fixed\", \"fontcolor\", \"fontsize\",\n  \"italics\", \"link\", \"small\", \"strike\", \"sub\", \"sup\"];\nfor (i = 0; i < names.length; i++) {\n  if (typeof String.prototype[names[i]] !== 'function') {\n    return false;\n  }\n}\nreturn true;",
            }, Subtest { name : "tags' names are lowercase", exec :
            "var i, names = [\"anchor\", \"big\", \"bold\", \"fixed\", \"fontcolor\", \"fontsize\",\n  \"italics\", \"link\", \"small\", \"strike\", \"sub\", \"sup\"];\nfor (i = 0; i < names.length; i++) {\n  if (\"\"[names[i]]().toLowerCase() !== \"\"[names[i]]()) {\n    return false;\n  }\n}\nreturn true;",
            }, Subtest { name : "quotes in arguments are escaped", exec :
            "var i, names = [\"anchor\", \"fontcolor\", \"fontsize\", \"link\"];\nfor (i = 0; i < names.length; i++) {\n  if (\"\"[names[i]]('\"') !== \"\"[names[i]]('&' + 'quot;')) {\n    return false;\n  }\n}\nreturn true;",
            },
        ]
    }
}
