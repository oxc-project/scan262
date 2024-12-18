use crate::feature::{Meta, Subtest};
use crate::features::Es6Let;
impl Meta for Es6Let {
    fn name(&self) -> &'static str {
        "let"
    }
    fn key(&self) -> &'static str {
        "es6_let"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "bindings"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-let-and-const-declarations"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/let"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic support", exec :
            "let foo = 123;\nreturn (foo === 123);" }, Subtest { name :
            "is block-scoped", exec :
            "let bar = 123;\n{ let bar = 456; }\nreturn bar === 123;" }, Subtest { name :
            "scope shadow resolution", exec :
            "try {\n    { let bar = 456; }\n    let bar = 123;\n    return bar === 123;\n} catch(e) {\n  return false;\n}"
            }, Subtest { name : "cannot be in statements", exec :
            "let bar = 1;\ntry {\n  Function(\"if(true) let baz = 1;\")();\n} catch(e) {\n  return true;\n}"
            }, Subtest { name : "for loop statement scope", exec :
            "let baz = 1;\nfor(let baz = 0; false;) {}\nreturn baz === 1;" }, Subtest {
            name : "temporal dead zone", exec :
            "var passed = (function(){ try {  qux; } catch(e) { return true; }}());\nfunction fn() { passed &= qux === 456; }\nlet qux = 456;\nfn();\nreturn passed;"
            }, Subtest { name : "for/for-in loop iteration scope", exec :
            "let scopes = [];\nfor(let i = 0; i < 2; i++) {\n  scopes.push(function(){ return i; });\n}\nlet passed = (scopes[0]() === 0 && scopes[1]() === 1);\n\nscopes = [];\nfor(let i in { a:1, b:1 }) {\n  scopes.push(function(){ return i; });\n}\npassed &= (scopes[0]() === \"a\" && scopes[1]() === \"b\");\nreturn passed;"
            }, Subtest { name : "for-in loop binding shadowing parameter", exec :
            "try {\n  Function(\"function f(e) { for (let e in {}) e }\");\n  return true;\n} catch(e) {\n  return false;\n}"
            }, Subtest { name : "basic support (strict mode)", exec :
            "'use strict';\nlet foo = 123;\nreturn (foo === 123);" }, Subtest { name :
            "is block-scoped (strict mode)", exec :
            "'use strict';\nlet bar = 123;\n{ let bar = 456; }\nreturn bar === 123;" },
            Subtest { name : "scope shadow resolution (strict mode)", exec :
            "'use strict';\ntry {\n    { let bar = 456; }\n    let bar = 123;\n    return bar === 123;\n} catch(e) {\n  return false;\n}"
            }, Subtest { name : "cannot be in statements (strict mode)", exec :
            "'use strict';\nlet bar = 1;\ntry {\n  Function(\"'use strict'; if(true) let baz = 1;\")();\n} catch(e) {\n  return true;\n}"
            }, Subtest { name : "for loop statement scope (strict mode)", exec :
            "'use strict';\nlet baz = 1;\nfor(let baz = 0; false;) {}\nreturn baz === 1;"
            }, Subtest { name : "temporal dead zone (strict mode)", exec :
            "'use strict';\nvar passed = (function(){ try {  qux; } catch(e) { return true; }}());\nfunction fn() { passed &= qux === 456; }\nlet qux = 456;\nfn();\nreturn passed;"
            }, Subtest { name : "for/for-in loop iteration scope (strict mode)", exec :
            "'use strict';\nlet scopes = [];\nfor(let i = 0; i < 2; i++) {\n  scopes.push(function(){ return i; });\n}\nlet passed = (scopes[0]() === 0 && scopes[1]() === 1);\n\nscopes = [];\nfor(let i in { a:1, b:1 }) {\n  scopes.push(function(){ return i; });\n}\npassed &= (scopes[0]() === \"a\" && scopes[1]() === \"b\");\nreturn passed;"
            }, Subtest { name : "for-in loop binding shadowing parameter (strict mode)",
            exec :
            "try {\n  Function(\"'use strict'; function f(e) { for (let e in {}) e }\");\n  return true;\n} catch(e) {\n  return false;\n}"
            },
        ]
    }
}
