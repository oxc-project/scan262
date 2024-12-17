use crate::feature::{Meta, Subtest};
use crate::features::Es6RegExpYAndUFlags;
impl Meta for Es6RegExpYAndUFlags {
    fn name(&self) -> &'static str {
        "RegExp \"y\" and \"u\" flags"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-get-regexp.prototype.sticky"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp#Parameters"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "\"y\" flag", exec :
            "var re = new RegExp('\\\\w', 'y');\nre.exec('xy');\nreturn (re.exec('xy')[0] === 'y');",
            }, Subtest { name : "\"y\" flag, lastIndex", exec :
            "var re = new RegExp('yy', 'y');\nre.lastIndex = 3;\nvar result = re.exec('xxxyyxx')[0];\nreturn result === 'yy' && re.lastIndex === 5;",
            }, Subtest { name : "\"u\" flag", exec :
            "return \"𠮷\".match(/^.$/u)[0].length === 2;", }, Subtest { name :
            "\"u\" flag, non-BMP Unicode characters", exec :
            "return \"𠮷x\".match(/^.x$/u)[0].length === 3;", }, Subtest { name :
            "\"u\" flag, Unicode code point escapes", exec :
            "return \"𝌆\".match(/\\u{1d306}/u)[0].length === 2;", }, Subtest { name :
            "\"u\" flag, case folding", exec :
            "return \"ſ\".match(/S/iu) && \"S\".match(/ſ/iu);", },
        ]
    }
}
