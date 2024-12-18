use crate::feature::{Meta, Subtest};
use crate::features::Es2024RegExpVFlag;
impl Meta for Es2024RegExpVFlag {
    fn name(&self) -> &'static str {
        "RegExp `v` flag"
    }
    fn key(&self) -> &'static str {
        "es2024_reg_exp_v_flag"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2024 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-v-flag"
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
            Subtest { name : "set notations", exec :
            "return /[\\p{ASCII}&&\\p{Decimal_Number}]/v.test(\"0\")\n&& /[\\p{Any}--[\\x01-\\u{10ffff}]]/v.test(\"\\0\")"
            }, Subtest { name : "properties of Strings", exec :
            "return /^\\p{Emoji_Keycap_Sequence}$/v.test(\"*\\uFE0F\\u20E3\")\n&& !/^\\p{Emoji_Keycap_Sequence}$/v.test(\"*\");"
            }, Subtest { name : "constructor supports it", exec :
            "return new RegExp('a', 'v') instanceof RegExp;" }, Subtest { name :
            "shows up in flags", exec :
            "var flags = [];\nvar p = new Proxy({}, { get: function (o, k) { flags.push(k); return o[k]; }});\nObject.getOwnPropertyDescriptor(RegExp.prototype, 'flags').get.call(p);\nreturn flags.indexOf(\"unicodeSets\") !== -1;"
            }, Subtest { name : "Unicode 15.1", exec :
            "return /^\\p{RGI_Emoji}$/v.test(\"🐦\u{200d}🔥\");" }, Subtest { name :
            "Unicode 16.0", exec : "return /^\\p{RGI_Emoji}$/v.test(\"🇨🇶\");" },
        ]
    }
}
