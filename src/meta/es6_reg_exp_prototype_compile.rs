use crate::feature::{Meta, Subtest};
use crate::features::Es6RegExpPrototypeCompile;
impl Meta for Es6RegExpPrototypeCompile {
    fn name(&self) -> &'static str {
        "RegExp.prototype.compile"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "annex b"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-regexp.prototype.compile"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/compile"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "if (typeof RegExp.prototype.compile !== 'function')\n  return false\nvar rx = /a/;\nrx.compile('b');\nreturn rx.test('b');",
            }, Subtest { name : "returns this", exec :
            "var rx = /a/;\nreturn rx.compile('b') === rx;", },
        ]
    }
}
