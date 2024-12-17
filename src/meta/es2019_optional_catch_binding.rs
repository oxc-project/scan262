use crate::feature::{Meta, Subtest};
use crate::features::Es2019OptionalCatchBinding;
impl Meta for Es2019OptionalCatchBinding {
    fn name(&self) -> &'static str {
        "optional catch binding"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 misc"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-optional-catch-binding"
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
            Subtest { name : "basic", exec :
            "try {\n  throw new Error();\n}\ncatch {\n  return true;\n}\nreturn false;",
            }, Subtest { name : "await", exec :
            "(async function () {\n  try {\n    await Promise.reject();\n  }\n  catch {\n    asyncTestPassed();\n  }\n})();",
            }, Subtest { name : "yield", exec :
            "function *foo() {\n  try {\n    yield;\n  }\n  catch {\n    return true;\n  }\n}\n\nvar it = foo();\nit.next();\nreturn it.throw().value;",
            },
        ]
    }
}
