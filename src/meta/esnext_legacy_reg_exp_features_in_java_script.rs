use crate::feature::{Meta, Subtest};
use crate::features::EsnextLegacyRegExpFeaturesInJavaScript;
impl Meta for EsnextLegacyRegExpFeaturesInJavaScript {
    fn name(&self) -> &'static str {
        "Legacy RegExp features in JavaScript"
    }
    fn key(&self) -> &'static str {
        "esnext_legacy_reg_exp_features_in_java_script"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 3"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-legacy-features"
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
            Subtest { name : "RegExp \"lastMatch\"", exec :
            "function () {\nvar re = /\\w/;\nre.exec('x');\nreturn RegExp.lastMatch === 'x';\n      }"
            }, Subtest { name : "RegExp.$1-$9", exec :
            "function () {\nfor (var i = 1; i < 10; i++) {\n  if (!(('$' + i) in RegExp)) return false;\n}\nreturn true;\n      }"
            },
        ]
    }
}
