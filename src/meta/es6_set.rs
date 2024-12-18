use crate::feature::{Meta, Subtest};
use crate::features::Es6Set;
impl Meta for Es6Set {
    fn name(&self) -> &'static str {
        "Set"
    }
    fn key(&self) -> &'static str {
        "es6_set"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-set-objects"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var obj = {};\nvar set = new Set();\n\nset.add(123);\nset.add(123);\n\nreturn set.has(123);",
            }, Subtest { name : "constructor arguments", exec :
            "var obj1 = {};\nvar obj2 = {};\nvar set = new Set([obj1, obj2]);\n\nreturn set.has(obj1) && set.has(obj2);",
            }, Subtest { name : "constructor requires new", exec :
            "new Set();\ntry {\n  Set();\n  return false;\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "constructor accepts null", exec :
            "new Set(null);\nreturn true;", }, Subtest { name :
            "constructor invokes add", exec :
            "var passed = false;\nvar _add = Set.prototype.add;\n\nSet.prototype.add = function(v) {\n  passed = true;\n};\n\nnew Set([1]);\nSet.prototype.add = _add;\n\nreturn passed;",
            }, Subtest { name : "iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\nvar add = Set.prototype.add;\nSet.prototype.add = function(){ throw 0 };\ntry {\n  new Set(iter);\n} catch(e){}\nSet.prototype.add = add;\nreturn closed;",
            }, Subtest { name : "Set.prototype.add returns this", exec :
            "var set = new Set();\nreturn set.add(0) === set;", }, Subtest { name :
            "-0 key converts to +0", exec :
            "var set = new Set();\nset.add(-0);\nvar k;\nset.forEach(function (value) {\n  k = 1 / value;\n});\nreturn k === Infinity && set.has(+0);",
            }, Subtest { name : "Set.prototype.size", exec :
            "var obj = {};\nvar set = new Set();\n\nset.add(123);\nset.add(123);\nset.add(456);\n\nreturn set.size === 2;",
            }, Subtest { name : "Set.prototype.delete", exec :
            "return typeof Set.prototype.delete === \"function\";", }, Subtest { name :
            "Set.prototype.clear", exec :
            "return typeof Set.prototype.clear === \"function\";", }, Subtest { name :
            "Set.prototype.forEach", exec :
            "return typeof Set.prototype.forEach === \"function\";", }, Subtest { name :
            "Set.prototype.keys", exec :
            "return typeof Set.prototype.keys === \"function\";", }, Subtest { name :
            "Set.prototype.values", exec :
            "return typeof Set.prototype.values === \"function\";", }, Subtest { name :
            "Set.prototype.entries", exec :
            "return typeof Set.prototype.entries === \"function\";", }, Subtest { name :
            "Set.prototype[Symbol.iterator]", exec :
            "return typeof Set.prototype[Symbol.iterator] === \"function\";", }, Subtest
            { name : "Set.prototype isn't an instance", exec :
            "new Set();\nvar obj = {};\ntry {\n  Set.prototype.has(obj);\n}\ncatch(e) {\n  return true;\n}",
            }, Subtest { name : "Set iterator prototype chain", exec :
            "// Iterator instance\nvar iterator = new Set()[Symbol.iterator]();\n// %SetIteratorPrototype%\nvar proto1 = Object.getPrototypeOf(iterator);\n// %IteratorPrototype%\nvar proto2 = Object.getPrototypeOf(proto1);\n\nreturn proto2.hasOwnProperty(Symbol.iterator) &&\n  !proto1    .hasOwnProperty(Symbol.iterator) &&\n  !iterator  .hasOwnProperty(Symbol.iterator) &&\n  iterator[Symbol.iterator]() === iterator;",
            }, Subtest { name : "Set[Symbol.species]", exec :
            "var prop = Object.getOwnPropertyDescriptor(Set, Symbol.species);\nreturn 'get' in prop && Set[Symbol.species] === Set;",
            },
        ]
    }
}
