use crate::feature::{Meta, Subtest};
use crate::features::Es6ArrowFunctions;
impl Meta for Es6ArrowFunctions {
    fn name(&self) -> &'static str {
        "arrow functions"
    }
    fn key(&self) -> &'static str {
        "es6_arrow_functions"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "functions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-arrow-function-definitions"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Arrow_functions"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "0 parameters", exec : "return (() => 5)() === 5;" },
            Subtest { name : "1 parameter, no brackets", exec :
            "var b = x => x + \"foo\";\nreturn (b(\"fee fie foe \") === \"fee fie foe foo\");"
            }, Subtest { name : "multiple parameters", exec :
            "var c = (v, w, x, y, z) => \"\" + v + w + x + y + z;\nreturn (c(6, 5, 4, 3, 2) === \"65432\");"
            }, Subtest { name : "lexical \"this\" binding", exec :
            "var d = { x : \"bar\", y : function() { return z => this.x + z; }}.y();\nvar e = { x : \"baz\", y : d };\nreturn d(\"ley\") === \"barley\" && e.y(\"ley\") === \"barley\";"
            }, Subtest { name : "\"this\" unchanged by call or apply", exec :
            "var d = { x : \"foo\", y : function() { return () => this.x; }};\nvar e = { x : \"bar\" };\nreturn d.y().call(e) === \"foo\" && d.y().apply(e) === \"foo\";"
            }, Subtest { name : "can't be bound, can be curried", exec :
            "var d = { x : \"bar\", y : function() { return z => this.x + z; }};\nvar e = { x : \"baz\" };\nreturn d.y().bind(e, \"ley\")() === \"barley\";"
            }, Subtest { name : "lexical \"arguments\" binding", exec :
            "var f = (function() { return z => arguments[0]; }(5));\nreturn f(6) === 5;"
            }, Subtest { name : "no line break between params and <code>=></code>", exec
            :
            "return (() => {\n  try { Function(\"x\\n => 2\")(); } catch(e) { return true; }\n})();"
            }, Subtest { name : "correct precedence", exec :
            "return (() => {\n  try { Function(\"0 || () => 2\")(); } catch(e) { return true; }\n})();"
            }, Subtest { name : "no \"prototype\" property", exec :
            "var a = () => 5;\nreturn !a.hasOwnProperty(\"prototype\");" }, Subtest {
            name : "lexical \"super\" binding in constructors", exec :
            "var received;\n\nclass B {\n  constructor (arg) {\n    received = arg;\n  }\n}\nclass C extends B {\n  constructor () {\n    var callSuper = () => super('foo');\n    callSuper();\n  }\n}\nreturn new C instanceof C && received === 'foo'"
            }, Subtest { name : "lexical \"super\" binding in methods", exec :
            "class B {\n  qux() {\n    return \"quux\";\n  }\n}\nclass C extends B {\n  baz() {\n    return x => super.qux();\n  }\n}\nvar arrow = new C().baz();\nreturn arrow() === \"quux\";"
            }, Subtest { name : "lexical \"new.target\" binding", exec :
            "function C() {\n  return x => new.target;\n}\nreturn new C()() === C && C()() === void undefined;"
            },
        ]
    }
}
