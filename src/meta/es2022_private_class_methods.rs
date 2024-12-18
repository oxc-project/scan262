use crate::feature::{Meta, Subtest};
use crate::features::Es2022PrivateClassMethods;
impl Meta for Es2022PrivateClassMethods {
    fn name(&self) -> &'static str {
        "private class methods"
    }
    fn key(&self) -> &'static str {
        "es2022_private_class_methods"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-private-methods"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "private instance methods", exec :
            "class C {\n  #x() { return 42; }\n  x() {\n    return this.#x();\n  }\n}\nreturn new C().x() === 42;"
            }, Subtest { name : "private static methods", exec :
            "class C {\n  static #x() { return 42; }\n  x() {\n    return C.#x();\n  }\n}\nreturn new C().x() === 42;"
            }, Subtest { name : "private accessor properties", exec :
            "var y = false;\nclass C {\n  get #x() { return 42; }\n  set #x(x) { y = x; }\n  x() {\n    this.#x = true;\n    return this.#x;\n  }\n}\nreturn new C().x() === 42 && y;"
            }, Subtest { name : "private static accessor properties", exec :
            "var y = false;\nclass C {\n  static get #x() { return 42; }\n  static set #x(x) { y = x; }\n  x() {\n    C.#x = true;\n    return C.#x;\n  }\n}\nreturn new C().x() === 42 && y;"
            },
        ]
    }
}
