use crate::feature::{Meta, Subtest};
use crate::features::Es2019FunctionPrototypeToStringRevision;
impl Meta for Es2019FunctionPrototypeToStringRevision {
    fn name(&self) -> &'static str {
        "Function.prototype.toString revision"
    }
    fn key(&self) -> &'static str {
        "es2019_function_prototype_to_string_revision"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 misc"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/Function-prototype-toString-revision"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/toString"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "functions created with the Function constructor", exec :
            "var fn = function ('a', ' /\\x2A a \\x2A/ b, c /\\x2A b \\x2A/ //', '/\\x2A c \\x2A/ ; /\\x2A d \\x2A/ //');\nvar str = 'function anonymous(a, /\\x2A a \\x2A/ b, c /\\x2A b \\x2A/ //\\n) {\\n/\\x2A c \\x2A/ ; /\\x2A d \\x2A/ //\\n}';\nreturn fn + '' === str;"
            }, Subtest { name : "arrows", exec :
            "var str = 'a => b';\nreturn eval('(' + str + ')') + '' === str;" }, Subtest
            { name : "[native code]", exec :
            "const NATIVE_EVAL_RE = /\\bfunction\\b[\\s\\S]*\\beval\\b[\\s\\S]*\\([\\s\\S]*\\)[\\s\\S]*\\{[\\s\\S]*\\[[\\s\\S]*\\bnative\\b[\\s\\S]+\\bcode\\b[\\s\\S]*\\][\\s\\S]*\\}/;\nreturn NATIVE_EVAL_RE.test(eval + '');"
            }, Subtest { name : "class expression with implicit constructor", exec :
            "var str = 'class A {}';\nreturn eval('(' + str + ')') + '' === str;" },
            Subtest { name : "class expression with explicit constructor", exec :
            "var str = 'class /\\x2A a \\x2A/ A /\\x2A b \\x2A/ extends /\\x2A c \\x2A/ function B() {} /\\x2A d \\x2A/ { /\\x2A e \\x2A/ constructor /\\x2A f \\x2A/ ( /\\x2A g \\x2A/ ) /\\x2A h \\x2A/ { /\\x2A i \\x2A/ ; /\\x2A j \\x2A/ } /\\x2A k \\x2A/ m /\\x2A l \\x2A/ ( /\\x2A m \\x2A/ ) /\\x2A n \\x2A/ { /\\x2A o \\x2A/ } /\\x2A p \\x2A/ }';\nreturn eval('(/\\x2A before \\x2A/' + str + '/\\x2A after \\x2A/)') + '' === str;"
            }, Subtest { name : "unicode escape sequences in identifiers", exec :
            "var str = 'function \\\\u0061(\\\\u{62}, \\\\u0063) { \\\\u0062 = \\\\u{00063}; return b; }';\nreturn eval('(/\\x2A before \\x2A/' + str + '/\\x2A after \\x2A/)') + '' === str;"
            }, Subtest { name : "methods and computed property names", exec :
            "var str = '[ /\\x2A a \\x2A/ \"f\" /\\x2A b \\x2A/ ] /\\x2A c \\x2A/ ( /\\x2A d \\x2A/ ) /\\x2A e \\x2A/ { /\\x2A f \\x2A/ }';\nreturn eval('({ /\\x2A before \\x2A/' + str + '/\\x2A after \\x2A/ }.f)') + '' === str;"
            },
        ]
    }
}
