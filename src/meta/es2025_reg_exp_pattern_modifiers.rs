use crate::{
    feature::{Meta, Subtest},
    features::Es2025RegExpPatternModifiers,
};
impl Meta for Es2025RegExpPatternModifiers {
    fn name(&self) -> &'static str {
        "RegExp Pattern Modifiers"
    }

    fn key(&self) -> &'static str {
        "es2025_reg_exp_pattern_modifiers"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2025 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-modifiers"
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "i flag",
                exec: "const regex = /^[a-z](?-i:[a-z])$/i;\nreturn regex.test(\"ab\") && regex.test(\"Ab\") && !regex.test(\"aB\");",
            },
            Subtest {
                name: "m flag",
                exec: "const regex = /^a|(?m:^b)/;\nreturn regex.test(\"a\") && regex.test(\"b\") && regex.test(\"c\\nb\") && !regex.test(\"c\\na\");",
            },
            Subtest {
                name: "s flag",
                exec: "const regex = /.(?-s:.)/s;\nreturn regex.test(\"\\na\") && regex.test(\"aa\") && !regex.test(\"\\n\\n\");",
            },
        ]
    }
}
