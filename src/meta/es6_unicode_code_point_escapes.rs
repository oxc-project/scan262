use crate::feature::{Meta, Subtest};
use crate::features::Es6UnicodeCodePointEscapes;
impl Meta for Es6UnicodeCodePointEscapes {
    fn name(&self) -> &'static str {
        "Unicode code point escapes"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-literals-string-literals"
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
            Subtest { name: "in strings", exec: "return '\\u{1d306}' === '\\ud834\\udf06';" },
            Subtest {
                name: "in identifiers",
                exec: "var \\u{102C0} = 2;\nreturn \\u{102C0} === 2;",
            },
            Subtest {
                name: "in property key definitions",
                exec: "var o = { \\u{102C0} : 2 };\nreturn o['\\ud800\\udec0'] === 2;",
            },
            Subtest {
                name: "in property key accesses",
                exec: "var o = { '\\ud800\\udec0' : 2 };\nreturn o.\\u{102C0} === 2;",
            },
        ]
    }
}
