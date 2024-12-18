use crate::feature::{Meta, Subtest};
use crate::features::Es2017ObjectPrototypeGetterSetterMethods;
impl Meta for Es2017ObjectPrototypeGetterSetterMethods {
    fn name(&self) -> &'static str {
        "Object.prototype getter/setter methods"
    }
    fn key(&self) -> &'static str {
        "es2017_object_prototype_getter_setter_methods"
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
            Subtest { name : "__defineGetter__", exec :
            "var obj = {};\nfunction bar() { return \"bar\"; }\nObject.prototype.__defineGetter__.call(obj, \"foo\", bar);\nvar prop = Object.getOwnPropertyDescriptor(obj, \"foo\");\nreturn prop.get === bar && !prop.writable && prop.configurable\n&& prop.enumerable;",
            }, Subtest { name : "__defineGetter__, symbols", exec :
            "var obj = {};\nvar sym = Symbol();\nfunction bar() { return \"bar\"; }\nObject.prototype.__defineGetter__.call(obj, sym, bar);\nvar prop = Object.getOwnPropertyDescriptor(obj, sym);\nreturn prop.get === bar && !prop.writable && prop.configurable\n&& prop.enumerable;",
            }, Subtest { name : "__defineGetter__, ToObject(this)", exec :
            "var key = '__accessors_test__';\n__defineGetter__.call(1, key, function () {});\ntry {\n  __defineGetter__.call(null, key, function () {});\n} catch (e) {\nreturn true;\n}",
            }, Subtest { name : "__defineSetter__", exec :
            "var obj = {};\nfunction bar() {}\nObject.prototype.__defineSetter__.call(obj, \"foo\", bar);\nvar prop = Object.getOwnPropertyDescriptor(obj, \"foo\");\nreturn prop.set === bar && !prop.writable && prop.configurable\n&& prop.enumerable;",
            }, Subtest { name : "__defineSetter__, symbols", exec :
            "var obj = {};\nvar sym = Symbol();\nfunction bar(baz) {}\nObject.prototype.__defineSetter__.call(obj, sym, bar);\nvar prop = Object.getOwnPropertyDescriptor(obj, sym);\nreturn prop.set === bar && !prop.writable && prop.configurable\n&& prop.enumerable;",
            }, Subtest { name : "__defineSetter__, ToObject(this)", exec :
            "var key = '__accessors_test__';\n__defineSetter__.call(1, key, function () {});\ntry {\n__defineSetter__.call(null, key, function () {});\n} catch (e) {\nreturn true;\n}",
            }, Subtest { name : "__lookupGetter__", exec :
            "var obj = {\nget foo() { return \"bar\"},\nqux: 1\n};\nvar foo = Object.prototype.__lookupGetter__.call(obj, \"foo\");\nreturn foo() === \"bar\"\n&& Object.prototype.__lookupGetter__.call(obj, \"qux\") === void undefined\n&& Object.prototype.__lookupGetter__.call(obj, \"baz\") === void undefined;",
            }, Subtest { name : "__lookupGetter__, prototype chain", exec :
            "var obj = {\nget foo() { return \"bar\"},\nqux: 1\n};\nvar foo = Object.prototype.__lookupGetter__.call(Object.create(obj), \"foo\");\nreturn foo() === \"bar\"\n&& Object.prototype.__lookupGetter__.call(obj, \"qux\") === void undefined\n&& Object.prototype.__lookupGetter__.call(obj, \"baz\") === void undefined;",
            }, Subtest { name : "__lookupGetter__, symbols", exec :
            "var sym = Symbol();\nvar sym2 = Symbol();\nvar obj = {};\nObject.defineProperty(obj, sym, { get: function () { return \"bar\"; }});\nObject.defineProperty(obj, sym2, { value: 1 });\nvar foo = Object.prototype.__lookupGetter__.call(obj, sym);\nreturn foo() === \"bar\"\n  && Object.prototype.__lookupGetter__.call(obj, sym2) === void undefined\n  && Object.prototype.__lookupGetter__.call(obj, Symbol()) === void undefined;",
            }, Subtest { name : "__lookupGetter__, ToObject(this)", exec :
            "__lookupGetter__.call(1, 'key');\ntry {\n  __lookupGetter__.call(null, 'key');\n} catch (e) {\n  return true;\n}",
            }, Subtest { name : "__lookupGetter__, data properties can shadow accessors",
            exec :
            "var a = {};\nvar b = Object.create(a);\nb.foo = 1;\na.__defineGetter__(\"foo\", function () {});\nreturn b.__lookupGetter__(\"foo\") === void undefined;",
            }, Subtest { name : "__lookupSetter__", exec :
            "var obj = {\n  set foo(baz) { return \"bar\"; },\n  qux: 1\n};\nvar foo = Object.prototype.__lookupSetter__.call(obj, \"foo\");\nreturn foo() === \"bar\"\n  && Object.prototype.__lookupSetter__.call(obj, \"qux\") === void undefined\n  && Object.prototype.__lookupSetter__.call(obj, \"baz\") === void undefined;",
            }, Subtest { name : "__lookupSetter__, prototype chain", exec :
            "var obj = {\n  set foo(baz) { return \"bar\"; },\n  qux: 1\n};\nvar foo = Object.prototype.__lookupSetter__.call(Object.create(obj), \"foo\");\nreturn foo() === \"bar\"\n  && Object.prototype.__lookupSetter__.call(obj, \"qux\") === void undefined\n  && Object.prototype.__lookupSetter__.call(obj, \"baz\") === void undefined;",
            }, Subtest { name : "__lookupSetter__, symbols", exec :
            "var sym = Symbol();\nvar sym2 = Symbol();\nvar obj = {};\nObject.defineProperty(obj, sym, { set: function (baz) { return \"bar\"; }});\nObject.defineProperty(obj, sym2, { value: 1 });\nvar foo = Object.prototype.__lookupSetter__.call(obj, sym);\nreturn foo() === \"bar\"\n  && Object.prototype.__lookupSetter__.call(obj, sym2) === void undefined\n  && Object.prototype.__lookupSetter__.call(obj, Symbol()) === void undefined;",
            }, Subtest { name : "__lookupSetter__, ToObject(this)", exec :
            "__lookupSetter__.call(1, 'key');\ntry {\n  __lookupSetter__.call(null, 'key');\n} catch (e) {\n  return true;\n}",
            }, Subtest { name : "__lookupSetter__, data properties can shadow accessors",
            exec :
            "var a = {};\nvar b = Object.create(a);\nb.foo = 1;\na.__defineSetter__(\"foo\", function () {})\nreturn b.__lookupSetter__(\"foo\") === void undefined",
            },
        ]
    }
}
