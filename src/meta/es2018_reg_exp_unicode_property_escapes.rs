use crate::feature::{Meta, Subtest};
use crate::features::Es2018RegExpUnicodePropertyEscapes;
impl Meta for Es2018RegExpUnicodePropertyEscapes {
    fn name(&self) -> &'static str {
        "RegExp Unicode Property Escapes"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-unicode-property-escapes"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions/Unicode_Property_Escapes"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic", exec :
            "const regexGreekSymbol = /\\p{Script=Greek}/u;\nreturn regexGreekSymbol.test('π');",
            }, Subtest { name : "Unicode 11", exec :
            "return /\\p{Extended_Pictographic}/u.test(\"\\xA9\") && /\\p{Emoji}/u.test(\"🥰\");",
            }, Subtest { name : "Unicode 12", exec :
            "return /\\p{Script=Elymaic}/u.test(\"\\u{10fe0}\") && /\\p{Emoji}/u.test(\"🥱\");",
            }, Subtest { name : "Unicode 12.1", exec :
            "return /\\p{Other_Symbol}/u.test(\"\\u32FF\");", }, Subtest { name :
            "Unicode 13", exec :
            "return /\\p{Script=Chorasmian}/u.test(\"\\u{10fb0}\") && /\\p{Emoji}/u.test(\"🥲\");",
            }, Subtest { name : "Unicode 14", exec :
            "return /\\p{Script=Vithkuqi}/u.test(\"\\u{10570}\") && /\\p{Emoji}/u.test(\"🫠\");",
            }, Subtest { name : "Unicode 15", exec :
            "return /\\p{Script=Kawi}/u.test(\"\\u{11f00}\") && /\\p{Emoji}/u.test(\"🫨\");",
            }, Subtest { name : "Unicode 15.1", exec :
            "return /\\p{Unified_Ideograph}/u.test(\"\\u{2ebf0}\");", }, Subtest { name :
            "Unicode 16.0", exec :
            "return /\\p{Script=Todhri}/u.test(\"\\u{105c0}\") && /\\p{Emoji}/u.test(\"🫩\");",
            },
        ]
    }
}
