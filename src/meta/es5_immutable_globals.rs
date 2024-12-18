use crate::feature::{Meta, Subtest};
use crate::features::Es5ImmutableGlobals;
impl Meta for Es5ImmutableGlobals {
    fn name(&self) -> &'static str {
        "Immutable globals"
    }
    fn key(&self) -> &'static str {
        "es5_immutable_globals"
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
            Subtest { name : "undefined", exec :
            "undefined = 12345;\nvar result = typeof undefined === 'undefined';\nundefined = void 0;\nreturn result;",
            }, Subtest { name : "NaN", exec :
            "NaN = false;\nvar result = typeof NaN === 'number';\nNaN = Math.sqrt(-1);\nreturn result;",
            }, Subtest { name : "Infinity", exec :
            "Infinity = false;\nvar result = typeof Infinity === 'number';\nInfinity = 1/0;\nreturn result;",
            },
        ]
    }
}
