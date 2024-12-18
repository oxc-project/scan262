use crate::{
    feature::{Meta, Subtest},
    features::Es6WeakMap,
};
impl Meta for Es6WeakMap {
    fn name(&self) -> &'static str {
        "WeakMap"
    }

    fn key(&self) -> &'static str {
        "es6_weak_map"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-ins"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-weakmap-objects"
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakMap"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "basic functionality",
                exec: "var key = {};\nvar weakmap = new WeakMap();\n\nweakmap.set(key, 123);\n\nreturn weakmap.has(key) && weakmap.get(key) === 123;",
            },
            Subtest {
                name: "constructor arguments",
                exec: "var key1 = {};\nvar key2 = {};\nvar weakmap = new WeakMap([[key1, 123], [key2, 456]]);\n\nreturn weakmap.has(key1) && weakmap.get(key1) === 123 &&\n       weakmap.has(key2) && weakmap.get(key2) === 456;",
            },
            Subtest {
                name: "constructor requires new",
                exec: "new WeakMap();\ntry {\n  WeakMap();\n  return false;\n} catch(e) {\n  return true;\n}",
            },
            Subtest { name: "constructor accepts null", exec: "new WeakMap(null);\nreturn true;" },
            Subtest {
                name: "constructor invokes set",
                exec: "var passed = false;\nvar _set = WeakMap.prototype.set;\n\nWeakMap.prototype.set = function(k, v) {\n  passed = true;\n};\n\nnew WeakMap([ [{ }, 42] ]);\nWeakMap.prototype.set = _set;\n\nreturn passed;",
            },
            Subtest {
                name: "frozen objects as keys",
                exec: "var f = Object.freeze({});\nvar m = new WeakMap;\nm.set(f, 42);\nreturn m.get(f) === 42;",
            },
            Subtest {
                name: "iterator closing",
                exec: "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\ntry {\n  new WeakMap(iter);\n} catch(e){}\nreturn closed;",
            },
            Subtest {
                name: "WeakMap.prototype.set returns this",
                exec: "var weakmap = new WeakMap();\nvar key = {};\nreturn weakmap.set(key, 0) === weakmap;",
            },
            Subtest {
                name: "WeakMap.prototype.delete",
                exec: "return typeof WeakMap.prototype.delete === \"function\";",
            },
            Subtest {
                name: "no WeakMap.prototype.clear method",
                exec: "if (!(\"clear\" in WeakMap.prototype)) {\n  return true;\n}\nvar m = new WeakMap();\nvar key = {};\nm.set(key, 2);\nm.clear();\nreturn m.has(key);",
            },
            Subtest {
                name: ".has, .get and .delete methods accept primitives",
                exec: "var m = new WeakMap;\nreturn m.has(1) === false\n  && m.get(1) === void undefined\n  && m.delete(1) === false;",
            },
            Subtest {
                name: "WeakMap.prototype isn't an instance",
                exec: "new WeakMap();\nvar obj = {};\ntry {\n  WeakMap.prototype.has(obj);\n}\ncatch(e) {\n  return true;\n}",
            },
        ]
    }
}
