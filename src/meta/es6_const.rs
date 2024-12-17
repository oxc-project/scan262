use crate::feature::{Meta, Subtest};
use crate::features::Es6Const;
impl Meta for Es6Const {
    fn name(&self) -> &'static str {
        "const"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/const"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic support", exec :
            "const foo = 123;\nreturn (foo === 123);", }, Subtest { name :
            "is block-scoped", exec :
            "const bar = 123;\n{ const bar = 456; }\nreturn bar === 123;", }, Subtest {
            name : "scope shadow resolution", exec :
            "try {\n    { const bar = 456; }\n    const bar = 123;\n    return bar === 123;\n} catch(e) {\n  return false;\n}",
            }, Subtest { name : "cannot be in statements", exec :
            "const bar = 1;\ntry {\n  Function(\"if(true) const baz = 1;\")();\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "redefining a const is an error", exec :
            "const baz = 1;\ntry {\n  Function(\"const foo = 1; foo = 2;\")();\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "for loop statement scope", exec :
            "const baz = 1;\nfor(const baz = 0; false;) {}\nreturn baz === 1;", },
            Subtest { name : "for-in loop iteration scope", exec :
            "var scopes = [];\nfor(const i in { a:1, b:1 }) {\n  scopes.push(function(){ return i; });\n}\nreturn (scopes[0]() === \"a\" && scopes[1]() === \"b\");",
            }, Subtest { name : "for-of loop iteration scope", exec :
            "var scopes = [];\nfor(const i of ['a','b']) {\n  scopes.push(function(){ return i; });\n}\nreturn (scopes[0]() === \"a\" && scopes[1]() === \"b\");",
            }, Subtest { name : "temporal dead zone", exec :
            "var passed = (function(){ try { qux; } catch(e) { return true; }}());\nfunction fn() { passed &= qux === 456; }\nconst qux = 456;\nfn();\nreturn passed;",
            }, Subtest { name : "basic support (strict mode)", exec :
            "\"use strict\";\nconst foo = 123;\nreturn (foo === 123);", }, Subtest { name
            : "is block-scoped (strict mode)", exec :
            "'use strict';\nconst bar = 123;\n{ const bar = 456; }\nreturn bar === 123;",
            }, Subtest { name : "scope shadow resolution (strict mode)", exec :
            "'use strict';\ntry {\n    { const bar = 456; }\n    const bar = 123;\n    return bar === 123;\n} catch(e) {\n  return false;\n}",
            }, Subtest { name : "cannot be in statements (strict mode)", exec :
            "'use strict';\nconst bar = 1;\ntry {\n  Function(\"'use strict'; if(true) const baz = 1;\")();\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "redefining a const (strict mode)", exec :
            "'use strict';\nconst baz = 1;\ntry {\n  Function(\"'use strict'; const foo = 1; foo = 2;\")();\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "for loop statement scope (strict mode)", exec :
            "'use strict';\nconst baz = 1;\nfor(const baz = 0; false;) {}\nreturn baz === 1;",
            }, Subtest { name : "for-in loop iteration scope (strict mode)", exec :
            "'use strict';\nvar scopes = [];\nfor(const i in { a:1, b:1 }) {\n  scopes.push(function(){ return i; });\n}\nreturn (scopes[0]() === \"a\" && scopes[1]() === \"b\");",
            }, Subtest { name : "for-of loop iteration scope (strict mode)", exec :
            "'use strict';\nvar scopes = [];\nfor(const i of ['a','b']) {\n  scopes.push(function(){ return i; });\n}\nreturn (scopes[0]() === \"a\" && scopes[1]() === \"b\");",
            }, Subtest { name : "temporal dead zone (strict mode)", exec :
            "'use strict';\nvar passed = (function(){ try { qux; } catch(e) { return true; }}());\nfunction fn() { passed &= qux === 456; }\nconst qux = 456;\nfn();\nreturn passed;",
            },
        ]
    }
}
