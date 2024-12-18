use crate::{
    feature::{Meta, Subtest},
    features::Es6Super,
};
impl Meta for Es6Super {
    fn name(&self) -> &'static str {
        "super"
    }

    fn key(&self) -> &'static str {
        "es6_super"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "functions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-super-keyword"
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/super"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "statement in constructors",
                exec: "var passed = false;\nclass B {\n  constructor(a) { passed = (a === \"barbaz\"); }\n}\nclass C extends B {\n  constructor(a) { super(\"bar\" + a); }\n}\nnew C(\"baz\");\nreturn passed;",
            },
            Subtest {
                name: "expression in constructors",
                exec: "class B {\n  constructor(a) { return [\"foo\" + a]; }\n}\nclass C extends B {\n  constructor(a) { return super(\"bar\" + a); }\n}\nreturn new C(\"baz\")[0] === \"foobarbaz\";",
            },
            Subtest {
                name: "in methods, property access",
                exec: "class B {}\nB.prototype.qux = \"foo\";\nB.prototype.corge = \"baz\";\nclass C extends B {\n  quux(a) { return super.qux + a + super[\"corge\"]; }\n}\nC.prototype.qux = \"garply\";\nreturn new C().quux(\"bar\") === \"foobarbaz\";",
            },
            Subtest {
                name: "in methods, method calls",
                exec: "class B {\n  qux(a) { return \"foo\" + a; }\n}\nclass C extends B {\n  qux(a) { return super.qux(\"bar\" + a); }\n}\nreturn new C().qux(\"baz\") === \"foobarbaz\";",
            },
            Subtest {
                name: "method calls use correct \"this\" binding",
                exec: "class B {\n  qux(a) { return this.foo + a; }\n}\nclass C extends B {\n  qux(a) { return super.qux(\"bar\" + a); }\n}\nvar obj = new C();\nobj.foo = \"foo\";\nreturn obj.qux(\"baz\") === \"foobarbaz\";",
            },
            Subtest {
                name: "constructor calls use correct \"new.target\" binding",
                exec: "var passed;\nclass B {\n  constructor() { passed = (new.target === C); }\n}\nclass C extends B {\n  constructor() { super(); }\n}\nnew C();\nreturn passed;",
            },
            Subtest {
                name: "is statically bound",
                exec: "class B {\n  qux() { return \"bar\"; }\n}\nclass C extends B {\n  qux() { return super.qux() + this.corge; }\n}\nvar obj = {\n  qux: C.prototype.qux,\n  corge: \"ley\"\n};\nreturn obj.qux() === \"barley\";",
            },
            Subtest {
                name: "super() invokes the correct constructor",
                exec: "// checks that super() is *not* a synonym of super.constructor()\nvar passed;\nclass B {\n    constructor() {\n        passed = true;\n    }\n};\nB.prototype.constructor = function () {\n    passed = false;\n};\nclass C extends B { };\nnew C;\nreturn passed;",
            },
        ]
    }
}
