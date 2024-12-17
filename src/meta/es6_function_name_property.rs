use crate::feature::{Meta, Subtest};
use crate::features::Es6FunctionNameProperty;
impl Meta for Es6FunctionNameProperty {
    fn name(&self) -> &'static str {
        "function \"name\" property"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-in extensions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-setfunctionname"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/name"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "function statements", exec :
            "function foo(){};\nreturn foo.name === 'foo' &&\n  (function(){}).name === '';",
            }, Subtest { name : "function expressions", exec :
            "return (function foo(){}).name === 'foo' &&\n  (function(){}).name === '';",
            }, Subtest { name : "new Function", exec :
            "return (new Function).name === \"anonymous\";", }, Subtest { name :
            "bound functions", exec :
            "function foo() {};\nreturn foo.bind({}).name === \"bound foo\" &&\n  (function(){}).bind({}).name === \"bound \";",
            }, Subtest { name : "variables (function)", exec :
            "var foo = function() {};\nvar bar = function baz() {};\nreturn foo.name === \"foo\" && bar.name === \"baz\";",
            }, Subtest { name : "object methods (function)", exec :
            "var o = { foo: function(){}, bar: function baz(){}};\no.qux = function(){};\nreturn o.foo.name === \"foo\" &&\n       o.bar.name === \"baz\" &&\n       o.qux.name === \"\";",
            }, Subtest { name : "accessor properties", exec :
            "var o = { get foo(){}, set foo(x){} };\nvar descriptor = Object.getOwnPropertyDescriptor(o, \"foo\");\nreturn descriptor.get.name === \"get foo\" &&\n       descriptor.set.name === \"set foo\";",
            }, Subtest { name : "shorthand methods", exec :
            "var o = { foo(){} };\nreturn o.foo.name === \"foo\";", }, Subtest { name :
            "shorthand methods (no lexical binding)", exec :
            "var f = \"foo\";\nreturn ({f() { return f; }}).f() === \"foo\";", }, Subtest
            { name : "symbol-keyed methods", exec :
            "var sym1 = Symbol(\"foo\");\nvar sym2 = Symbol();\nvar o = {\n  [sym1]: function(){},\n  [sym2]: function(){}\n};\n\nreturn o[sym1].name === \"[foo]\" &&\n       o[sym2].name === \"\";",
            }, Subtest { name : "class statements", exec :
            "class foo {};\nclass bar { static name() {} };\nreturn foo.name === \"foo\" &&\n  typeof bar.name === \"function\";",
            }, Subtest { name : "class expressions", exec :
            "return class foo {}.name === \"foo\" &&\n  typeof class bar { static name() {} }.name === \"function\";",
            }, Subtest { name : "variables (class)", exec :
            "var foo = class {};\nvar bar = class baz {};\nvar qux = class { static name() {} };\nreturn foo.name === \"foo\" &&\n       bar.name === \"baz\" &&\n       typeof qux.name === \"function\";",
            }, Subtest { name : "object methods (class)", exec :
            "var o = { foo: class {}, bar: class baz {}};\no.qux = class {};\nreturn o.foo.name === \"foo\" &&\n       o.bar.name === \"baz\" &&\n       o.qux.name === \"\";",
            }, Subtest { name : "class prototype methods", exec :
            "class C { foo(){} };\nreturn (new C).foo.name === \"foo\";", }, Subtest {
            name : "class static methods", exec :
            "class C { static foo(){} };\nreturn C.foo.name === \"foo\";", }, Subtest {
            name : "isn't writable, is configurable", exec :
            "var descriptor = Object.getOwnPropertyDescriptor(function f(){},\"name\");\nreturn descriptor.enumerable   === false &&\n       descriptor.writable     === false &&\n       descriptor.configurable === true;",
            },
        ]
    }
}
