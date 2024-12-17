use crate::feature::{Meta, Subtest};
use crate::features::Es2017ObjectStaticMethods;
impl Meta for Es2017ObjectStaticMethods {
    fn name(&self) -> &'static str {
        "Object static methods"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 features"
    }
    fn spec(&self) -> &'static str {
        "https://tc39.github.io/ecma262/#sec-properties-of-the-object-constructor"
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
            Subtest { name : "Object.values", exec :
            "var obj = Object.create({ a: \"qux\", d: \"qux\" });\nobj.a = \"foo\";\nobj.b = \"bar\";\nobj.c = \"baz\";\nvar v = Object.values(obj);\nreturn Array.isArray(v) && String(v) === \"foo,bar,baz\";",
            }, Subtest { name : "Object.entries", exec :
            "var obj = Object.create({ a: \"qux\", d: \"qux\" });\nobj.a = \"foo\";\nobj.b = \"bar\";\nobj.c = \"baz\";\nvar e = Object.entries(obj);\nreturn Array.isArray(e)\n  && e.length === 3\n  && String(e[0]) === \"a,foo\"\n  && String(e[1]) === \"b,bar\"\n  && String(e[2]) === \"c,baz\";",
            }, Subtest { name : "Object.getOwnPropertyDescriptors", exec :
            "var object = {a: 1};\nvar B = typeof Symbol === 'function' ? Symbol('b') : 'b';\nobject[B] = 2;\nvar O = Object.defineProperty(object, 'c', {value: 3});\nvar D = Object.getOwnPropertyDescriptors(O);\n\nreturn D.a.value === 1 && D.a.enumerable === true && D.a.configurable === true && D.a.writable === true\n  && D[B].value === 2 && D[B].enumerable === true && D[B].configurable === true && D[B].writable === true\n  && D.c.value === 3 && D.c.enumerable === false && D.c.configurable === false && D.c.writable === false;",
            }, Subtest { name :
            "Object.getOwnPropertyDescriptors doesn't provide undefined descriptors",
            exec :
            "var P = new Proxy({ a: 1 }, {\n  getOwnPropertyDescriptor: function (t, k) {}\n});\nreturn !Object.getOwnPropertyDescriptors(P).hasOwnProperty('a');",
            },
        ]
    }
}
