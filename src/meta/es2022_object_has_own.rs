use crate::feature::{Meta, Subtest};
use crate::features::Es2022ObjectHasOwn;
impl Meta for Es2022ObjectHasOwn {
    fn name(&self) -> &'static str {
        "Object.hasOwn"
    }
    fn key(&self) -> &'static str {
        "es2022_object_has_own"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-accessible-object-hasownproperty"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Object/hasOwn"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Basic functionality", exec :
            "return Object.hasOwn({ x: 2 }, \"x\") === true;" }, Subtest { name :
            "ToObject called before ToPropertyKey", exec :
            "var ok = !!Object.hasOwn;\ntry {\n  Object.hasOwn(null, { toString: function () { ok = false } });\n  return false;\n} catch (e) {\n  return ok;\n}"
            },
        ]
    }
}
