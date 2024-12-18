use crate::feature::{Meta, Subtest};
use crate::features::Es6RegExpIsSubclassable;
impl Meta for Es6RegExpIsSubclassable {
    fn name(&self) -> &'static str {
        "RegExp is subclassable"
    }
    fn key(&self) -> &'static str {
        "es6_reg_exp_is_subclassable"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "subclassing"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-regexp-constructor"
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
            Subtest { name : "basic functionality", exec :
            "class R extends RegExp {}\nvar r = new R(\"baz\",\"g\");\nreturn r.global && r.source === \"baz\";"
            }, Subtest { name : "correct prototype chain", exec :
            "class R extends RegExp {}\nvar r = new R(\"baz\",\"g\");\nreturn r instanceof R && r instanceof RegExp && Object.getPrototypeOf(R) === RegExp;"
            }, Subtest { name : "RegExp.prototype.exec", exec :
            "class R extends RegExp {}\nvar r = new R(\"baz\",\"g\");\nreturn r.exec(\"foobarbaz\")[0] === \"baz\" && r.lastIndex === 9;"
            }, Subtest { name : "RegExp.prototype.test", exec :
            "class R extends RegExp {}\nvar r = new R(\"baz\");\nreturn r.test(\"foobarbaz\");"
            },
        ]
    }
}
