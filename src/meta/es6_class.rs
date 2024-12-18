use crate::feature::{Meta, Subtest};
use crate::features::Es6Class;
impl Meta for Es6Class {
    fn name(&self) -> &'static str {
        "class"
    }
    fn key(&self) -> &'static str {
        "es6_class"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "functions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-class-definitions"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "class statement", exec :
            "class C {}\nreturn typeof C === \"function\";", }, Subtest { name :
            "is block-scoped", exec :
            "class C {}\nvar c1 = C;\n{\n  class C {}\n  var c2 = C;\n}\nreturn C === c1;",
            }, Subtest { name : "class expression", exec :
            "return typeof class C {} === \"function\";", }, Subtest { name :
            "anonymous class", exec : "return typeof class {} === \"function\";", },
            Subtest { name : "constructor", exec :
            "class C {\n  constructor() { this.x = 1; }\n}\nreturn C.prototype.constructor === C\n  && new C().x === 1;",
            }, Subtest { name : "prototype methods", exec :
            "class C {\n  method() { return 2; }\n}\nreturn typeof C.prototype.method === \"function\"\n  && new C().method() === 2;",
            }, Subtest { name : "string-keyed methods", exec :
            "class C {\n  \"foo bar\"() { return 2; }\n}\nreturn typeof C.prototype[\"foo bar\"] === \"function\"\n  && new C()[\"foo bar\"]() === 2;",
            }, Subtest { name : "computed prototype methods", exec :
            "var foo = \"method\";\nclass C {\n  [foo]() { return 2; }\n}\nreturn typeof C.prototype.method === \"function\"\n  && new C().method() === 2;",
            }, Subtest { name : "optional semicolons", exec :
            "class C {\n  ;\n  method() { return 2; };\n  method2() { return 2; }\n  method3() { return 2; };\n}\nreturn typeof C.prototype.method === \"function\"\n  && typeof C.prototype.method2 === \"function\"\n  && typeof C.prototype.method3 === \"function\";",
            }, Subtest { name : "static methods", exec :
            "class C {\n  static method() { return 3; }\n}\nreturn typeof C.method === \"function\"\n  && C.method() === 3;",
            }, Subtest { name : "computed static methods", exec :
            "var foo = \"method\";\nclass C {\n  static [foo]() { return 3; }\n}\nreturn typeof C.method === \"function\"\n  && C.method() === 3;",
            }, Subtest { name : "accessor properties", exec :
            "var baz = false;\nclass C {\n  get foo() { return \"foo\"; }\n  set bar(x) { baz = x; }\n}\nnew C().bar = true;\nreturn new C().foo === \"foo\" && baz;",
            }, Subtest { name : "computed accessor properties", exec :
            "var garply = \"foo\", grault = \"bar\", baz = false;\nclass C {\n  get [garply]() { return \"foo\"; }\n  set [grault](x) { baz = x; }\n}\nnew C().bar = true;\nreturn new C().foo === \"foo\" && baz;",
            }, Subtest { name : "static accessor properties", exec :
            "var baz = false;\nclass C {\n  static get foo() { return \"foo\"; }\n  static set bar(x) { baz = x; }\n}\nC.bar = true;\nreturn C.foo === \"foo\" && baz;",
            }, Subtest { name : "computed static accessor properties", exec :
            "var garply = \"foo\", grault = \"bar\", baz = false;\nclass C {\n  static get [garply]() { return \"foo\"; }\n  static set [grault](x) { baz = x; }\n}\nC.bar = true;\nreturn C.foo === \"foo\" && baz;",
            }, Subtest { name : "class name is lexically scoped", exec :
            "class C {\n  method() { return typeof C === \"function\"; }\n}\nvar M = C.prototype.method;\nC = void undefined;\nreturn C === void undefined && M();",
            }, Subtest { name : "computed names, temporal dead zone", exec :
            "try {\n  var B = class C {\n    [C](){}\n  }\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "methods aren't enumerable", exec :
            "class C {\n  foo() {}\n  static bar() {}\n}\nreturn !C.prototype.propertyIsEnumerable(\"foo\") && !C.propertyIsEnumerable(\"bar\");",
            }, Subtest { name : "implicit strict mode", exec :
            "class C {\n  static method() { return this === void undefined; }\n}\nreturn (0,C.method)();",
            }, Subtest { name : "constructor requires new", exec :
            "class C {}\ntry {\n  C();\n}\ncatch(e) {\n  return true;\n}", }, Subtest {
            name : "extends", exec :
            "class B {}\nclass C extends B {}\nreturn new C() instanceof B\n  && B.isPrototypeOf(C);",
            }, Subtest { name : "extends expressions", exec :
            "var B;\nclass C extends (B = class {}) {}\nreturn new C() instanceof B\n  && B.isPrototypeOf(C);",
            }, Subtest { name : "extends null", exec :
            "class C extends null {\n  constructor() { return Object.create(null); }\n}\nreturn Function.prototype.isPrototypeOf(C)\n  && Object.getPrototypeOf(C.prototype) === null;",
            }, Subtest { name : "new.target", exec :
            "var passed = false;\nnew function f() {\n  passed = new.target === f;\n}();\n\nclass A {\n  constructor() {\n    passed &= new.target === B;\n  }\n}\nclass B extends A {}\nnew B();\nreturn passed;",
            },
        ]
    }
}
