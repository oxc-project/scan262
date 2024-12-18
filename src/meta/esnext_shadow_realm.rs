use crate::feature::{Meta, Subtest};
use crate::features::EsnextShadowRealm;
impl Meta for EsnextShadowRealm {
    fn name(&self) -> &'static str {
        "ShadowRealm"
    }
    fn key(&self) -> &'static str {
        "esnext_shadow_realm"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 2.7"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-shadowrealm"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "return typeof ShadowRealm === \"function\"\n  && [\"evaluate\", \"importValue\"].every(function(key){\n    return key in ShadowRealm.prototype;\n  });"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
