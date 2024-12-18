use crate::{
    feature::{Meta, Subtest},
    features::Es2017ProxyInternalCallsGetterSetterMethods,
};
impl Meta for Es2017ProxyInternalCallsGetterSetterMethods {
    fn name(&self) -> &'static str {
        "Proxy internal calls, getter/setter methods"
    }

    fn key(&self) -> &'static str {
        "es2017_proxy_internal_calls_getter_setter_methods"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2017 annex b"
    }

    fn spec(&self) -> &'static str {
        "https://tc39.github.io/ecma262/#sec-object.prototype.__defineGetter__"
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
            Subtest {
                name: "__defineGetter__",
                exec: "// Object.prototype.__defineGetter__ -> DefinePropertyOrThrow -> [[DefineOwnProperty]]\nvar def = [];\nvar p = new Proxy({}, {\n  defineProperty: function (o, v, desc) {\n    def.push(v);\n    Object.defineProperty(o, v, desc);\n    return true;\n  }\n});\nObject.prototype.__defineGetter__.call(p, \"foo\", Object);\nreturn def + '' === \"foo\";",
            },
            Subtest {
                name: "__defineSetter__",
                exec: "// Object.prototype.__defineSetter__ -> DefinePropertyOrThrow -> [[DefineOwnProperty]]\nvar def = [];\nvar p = new Proxy({}, {\n  defineProperty: function (o, v, desc) {\n    def.push(v);\n    Object.defineProperty(o, v, desc);\n    return true;\n  }\n});\nObject.prototype.__defineSetter__.call(p, \"foo\", Object);\nreturn def + '' === \"foo\";",
            },
            Subtest {
                name: "__lookupGetter__",
                exec: "// Object.prototype.__lookupGetter__ -> [[GetOwnProperty]]\n// Object.prototype.__lookupGetter__ -> [[GetPrototypeOf]]\nvar gopd = [];\nvar gpo = false;\nvar p = new Proxy({}, {\n  getPrototypeOf: function (o) {\n    gpo = true;\n    return Object.getPrototypeOf(o);\n  },\n  getOwnPropertyDescriptor: function (o, v) {\n    gopd.push(v);\n    return Object.getOwnPropertyDescriptor(o, v);\n  }\n});\nObject.prototype.__lookupGetter__.call(p, \"foo\");\nreturn gopd + '' === \"foo\" && gpo;",
            },
            Subtest {
                name: "__lookupSetter__",
                exec: "// Object.prototype.__lookupSetter__ -> [[GetOwnProperty]]\n// Object.prototype.__lookupSetter__ -> [[GetPrototypeOf]]\nvar gopd = [];\nvar gpo = false;\nvar p = new Proxy({}, {\n  getPrototypeOf: function (o) {\n    gpo = true;\n    return Object.getPrototypeOf(o);\n  },\n  getOwnPropertyDescriptor: function (o, v) {\n    gopd.push(v);\n    return Object.getOwnPropertyDescriptor(o, v);\n  }\n});\nObject.prototype.__lookupSetter__.call(p, \"foo\");\nreturn gopd + '' === \"foo\" && gpo;",
            },
        ]
    }
}
