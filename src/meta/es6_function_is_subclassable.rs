use crate::feature::{Meta, Subtest};
use crate::features::Es6FunctionIsSubclassable;
impl Meta for Es6FunctionIsSubclassable {
    fn name(&self) -> &'static str {
        "Function is subclassable"
    }
    fn key(&self) -> &'static str {
        "es6_function_is_subclassable"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "subclassing"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-function-constructor"
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
            Subtest { name : "can be called", exec :
            "class C extends Function {}\nvar c = new C(\"return 'foo';\");\nreturn c() === 'foo';"
            }, Subtest { name : "correct prototype chain", exec :
            "class C extends Function {}\nvar c = new C(\"return 'foo';\");\nreturn c instanceof C && c instanceof Function && Object.getPrototypeOf(C) === Function;"
            }, Subtest { name : "can be used with \"new\"", exec :
            "class C extends Function {}\nvar c = new C(\"this.bar = 2;\");\nc.prototype.baz = 3;\nreturn new c().bar === 2 && new c().baz === 3;"
            }, Subtest { name : "Function.prototype.call", exec :
            "class C extends Function {}\nvar c = new C(\"x\", \"return this.bar + x;\");\nreturn c.call({bar:1}, 2) === 3;"
            }, Subtest { name : "Function.prototype.apply", exec :
            "class C extends Function {}\nvar c = new C(\"x\", \"return this.bar + x;\");\nreturn c.apply({bar:1}, [2]) === 3;"
            }, Subtest { name : "Function.prototype.bind", exec :
            "class C extends Function {}\nvar c = new C(\"x\", \"y\", \"return this.bar + x + y;\").bind({bar:1}, 2);\nreturn c(6) === 9 && c instanceof C;"
            },
        ]
    }
}
