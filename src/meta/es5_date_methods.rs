use crate::feature::{Meta, Subtest};
use crate::features::Es5DateMethods;
impl Meta for Es5DateMethods {
    fn name(&self) -> &'static str {
        "Date methods"
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
            Subtest { name : "Date.prototype.toISOString", exec :
            "function () {\nreturn typeof Date.prototype.toISOString === 'function';\n    }",
            }, Subtest { name : "Date.now", exec :
            "function () {\nreturn typeof Date.now === 'function';\n    }", }, Subtest {
            name : "Date.prototype.toJSON", exec :
            "function () {\ntry {\n  return Date.prototype.toJSON.call(new Date(NaN)) === null;\n} catch (e) {\n  return false;\n}\n    }",
            },
        ]
    }
}
