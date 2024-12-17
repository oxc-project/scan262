use crate::feature::{Meta, Subtest};
use crate::features::Es2021WeakReferences;
impl Meta for Es2021WeakReferences {
    fn name(&self) -> &'static str {
        "WeakReferences"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2021 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-weakrefs"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakRef"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "WeakRef minimal support", exec :
            "var O = {};\nvar weakref = new WeakRef(O);\nreturn weakref.deref() === O;",
            }, Subtest { name : "FinalizationRegistry minimal support", exec :
            "var fr = new FinalizationRegistry(function () {});\nreturn Object.getPrototypeOf(fr) === FinalizationRegistry.prototype;",
            },
        ]
    }
}
