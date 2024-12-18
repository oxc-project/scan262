use crate::feature::{Meta, Subtest};
use crate::features::Es6RegExpSyntaxExtensions;
impl Meta for Es6RegExpSyntaxExtensions {
    fn name(&self) -> &'static str {
        "RegExp syntax extensions"
    }
    fn key(&self) -> &'static str {
        "es6_reg_exp_syntax_extensions"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "annex b"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-regular-expressions-patterns"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "hyphens in character sets", exec :
            "return /[\\w-_]/.exec(\"-\")[0] === \"-\";", }, Subtest { name :
            "invalid character escapes", exec :
            "return /\\z/.exec(\"\\\\z\")[0] === \"z\"\n  && /[\\z]/.exec(\"[\\\\z]\")[0] === \"z\";",
            }, Subtest { name : "invalid control-character escapes", exec :
            "return /\\c2/.exec(\"\\\\c2\")[0] === \"\\\\c2\";", }, Subtest { name :
            "invalid Unicode escapes", exec :
            "return /\\u1/.exec(\"u1\")[0] === \"u1\"\n  && /[\\u1]/.exec(\"u\")[0] === \"u\";",
            }, Subtest { name : "invalid hexadecimal escapes", exec :
            "return /\\x1/.exec(\"x1\")[0] === \"x1\"\n  && /[\\x1]/.exec(\"x\")[0] === \"x\";",
            }, Subtest { name : "incomplete patterns and quantifiers", exec :
            "return /x{1/.exec(\"x{1\")[0] === \"x{1\"\n  && /x]1/.exec(\"x]1\")[0] === \"x]1\";",
            }, Subtest { name : "octal escape sequences", exec :
            "return /\\041/.exec(\"!\")[0] === \"!\"\n  && /[\\041]/.exec(\"!\")[0] === \"!\";",
            }, Subtest { name : "invalid backreferences become octal escapes", exec :
            "return /\\41/.exec(\"!\")[0] === \"!\"\n  && /[\\41]/.exec(\"!\")[0] === \"!\";",
            },
        ]
    }
}
