use crate::feature::{Meta, Subtest};
use crate::features::Es2022InstanceClassFields;
impl Meta for Es2022InstanceClassFields {
    fn name(&self) -> &'static str {
        "instance class fields"
    }
    fn key(&self) -> &'static str {
        "es2022_instance_class_fields"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-class-fields"
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
            Subtest { name : "public instance class fields", exec :
            "class C {\n  x = 'x';\n}\nreturn new C().x === 'x';", }, Subtest { name :
            "private instance class fields basic support", exec :
            "class C {\n  #x;\n  constructor(x) {\n    this.#x = x;\n  }\n  x() {\n    return this.#x;\n  }\n}\nreturn new C(42).x() === 42;",
            }, Subtest { name : "private instance class fields initializers", exec :
            "class C {\n  #x = 42;\n  x() {\n    return this.#x;\n  }\n}\nreturn new C().x() === 42;",
            }, Subtest { name : "optional private instance class fields access", exec :
            "class C {\n  #x = 42;\n  x(o = this) {\n    return o?.#x;\n  }\n}\nreturn new C().x() === 42 && new C().x(null) === void undefined;",
            }, Subtest { name : "optional deep private instance class fields access",
            exec :
            "class C {\n  #x = 42;\n  x(o = { p: this }) {\n    return o?.p.#x;\n  }\n}\nreturn new C().x() === 42 && new C().x(null) === void undefined;",
            }, Subtest { name : "computed instance class fields", exec :
            "class C {\n  ['x'] = 42;\n}\nreturn new C().x === 42;", }, Subtest { name :
            "resolving identifier in parent scope", exec :
            "{\n  let a = [\"hello world\"];\n  class MyClass {\n    // The parenthesis below are required to trigger https://bugs.webkit.org/show_bug.cgi?id=236843\n    c = a[(0)];\n  }\n  return new MyClass().c === a[0];\n}",
            },
        ]
    }
}
