use crate::feature::{Meta, Subtest};
use crate::features::Es2020GlobalThis;
impl Meta for Es2020GlobalThis {
    fn name(&self) -> &'static str {
        "globalThis"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-global"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/globalThis"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "\"globalThis\" global property is global object", exec :
            "var actualGlobal = function ('return this')();\nactualGlobal.__system_global_test__ = 42;\nreturn typeof globalThis === 'object' && globalThis && globalThis === actualGlobal && !globalThis.lacksGlobalThis && globalThis.__system_global_test__ === 42;",
            }, Subtest { name :
            "\"globalThis\" global property has correct property descriptor", exec :
            "var actualGlobal = function ('return this')();\nif (typeof globalThis !== 'object') { return false; }\nif (!('globalThis' in actualGlobal)) { return false; }\nif (Object.prototype.propertyIsEnumerable.call(actualGlobal, 'globalThis')) { return false; }\n\nvar descriptor = Object.getOwnPropertyDescriptor(actualGlobal, 'globalThis');\nreturn descriptor.value === actualGlobal && !descriptor.enumerable && descriptor.configurable && descriptor.writable;",
            },
        ]
    }
}
