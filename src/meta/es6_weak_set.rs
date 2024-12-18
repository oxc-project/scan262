use crate::feature::{Meta, Subtest};
use crate::features::Es6WeakSet;
impl Meta for Es6WeakSet {
    fn name(&self) -> &'static str {
        "WeakSet"
    }
    fn key(&self) -> &'static str {
        "es6_weak_set"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-weakset-objects"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/WeakSet"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var obj1 = {};\nvar weakset = new WeakSet();\n\nweakset.add(obj1);\nweakset.add(obj1);\n\nreturn weakset.has(obj1);",
            }, Subtest { name : "constructor arguments", exec :
            "var obj1 = {}, obj2 = {};\nvar weakset = new WeakSet([obj1, obj2]);\n\nreturn weakset.has(obj1) && weakset.has(obj2);",
            }, Subtest { name : "constructor requires new", exec :
            "new WeakSet();\ntry {\n  WeakSet();\n  return false;\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "constructor accepts null", exec :
            "new WeakSet(null);\nreturn true;", }, Subtest { name :
            "constructor invokes add", exec :
            "var passed = false;\nvar _add = WeakSet.prototype.add;\n\nWeakSet.prototype.add = function(v) {\n  passed = true;\n};\n\nnew WeakSet([ { } ]);\nWeakSet.prototype.add = _add;\n\nreturn passed;",
            }, Subtest { name : "iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\ntry {\n  new WeakSet(iter);\n} catch(e){}\nreturn closed;",
            }, Subtest { name : "WeakSet.prototype.add returns this", exec :
            "var weakset = new WeakSet();\nvar obj = {};\nreturn weakset.add(obj) === weakset;",
            }, Subtest { name : "WeakSet.prototype.delete", exec :
            "return typeof WeakSet.prototype.delete === \"function\";", }, Subtest { name
            : "no WeakSet.prototype.clear method", exec :
            "if (!(\"clear\" in WeakSet.prototype)) {\n  return true;\n}\nvar s = new WeakSet();\nvar key = {};\ns.add(key);\ns.clear();\nreturn s.has(key);",
            }, Subtest { name : ".has and .delete methods accept primitives", exec :
            "var s = new WeakSet;\nreturn s.has(1) === false\n  && s.delete(1) === false;",
            }, Subtest { name : "WeakSet.prototype isn't an instance", exec :
            "new WeakSet();\nvar obj = {};\ntry {\n  WeakSet.prototype.has(obj);\n}\ncatch(e) {\n  return true;\n}",
            },
        ]
    }
}
