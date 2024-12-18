use crate::feature::{Meta, Subtest};
use crate::features::Es6Map;
impl Meta for Es6Map {
    fn name(&self) -> &'static str {
        "Map"
    }
    fn key(&self) -> &'static str {
        "es6_map"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-map-objects"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var key = {};\nvar map = new Map();\n\nmap.set(key, 123);\n\nreturn map.has(key) && map.get(key) === 123;"
            }, Subtest { name : "constructor arguments", exec :
            "var key1 = {};\nvar key2 = {};\nvar map = new Map([[key1, 123], [key2, 456]]);\n\nreturn map.has(key1) && map.get(key1) === 123 &&\n       map.has(key2) && map.get(key2) === 456;"
            }, Subtest { name : "constructor requires new", exec :
            "new Map();\ntry {\n  Map();\n  return false;\n} catch(e) {\n  return true;\n}"
            }, Subtest { name : "constructor accepts null", exec :
            "new Map(null);\nreturn true;" }, Subtest { name : "constructor invokes set",
            exec :
            "var passed = false;\nvar _set = Map.prototype.set;\n\nMap.prototype.set = function(k, v) {\n  passed = true;\n};\n\nnew Map([ [1, 2] ]);\nMap.prototype.set = _set;\n\nreturn passed;"
            }, Subtest { name : "iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\ntry {\n  new Map(iter);\n} catch(e){}\nreturn closed;"
            }, Subtest { name : "Map.prototype.set returns this", exec :
            "var map = new Map();\nreturn map.set(0, 0) === map;" }, Subtest { name :
            "-0 key converts to +0", exec :
            "var map = new Map();\nmap.set(-0, \"foo\");\nvar k;\nmap.forEach(function (value, key) {\n  k = 1 / key;\n});\nreturn k === Infinity && map.get(+0) === \"foo\";"
            }, Subtest { name : "Map.prototype.size", exec :
            "var key = {};\nvar map = new Map();\n\nmap.set(key, 123);\n\nreturn map.size === 1;"
            }, Subtest { name : "Map.prototype.delete", exec :
            "return typeof Map.prototype.delete === \"function\";" }, Subtest { name :
            "Map.prototype.clear", exec :
            "return typeof Map.prototype.clear === \"function\";" }, Subtest { name :
            "Map.prototype.forEach", exec :
            "return typeof Map.prototype.forEach === \"function\";" }, Subtest { name :
            "Map.prototype.keys", exec :
            "return typeof Map.prototype.keys === \"function\";" }, Subtest { name :
            "Map.prototype.values", exec :
            "return typeof Map.prototype.values === \"function\";" }, Subtest { name :
            "Map.prototype.entries", exec :
            "return typeof Map.prototype.entries === \"function\";" }, Subtest { name :
            "Map.prototype[Symbol.iterator]", exec :
            "return typeof Map.prototype[Symbol.iterator] === \"function\";" }, Subtest {
            name : "Map.prototype isn't an instance", exec :
            "new Map();\nvar obj = {};\ntry {\n  Map.prototype.has(obj);\n}\ncatch(e) {\n  return true;\n}"
            }, Subtest { name : "Map iterator prototype chain", exec :
            "// Iterator instance\nvar iterator = new Map()[Symbol.iterator]();\n// %MapIteratorPrototype%\nvar proto1 = Object.getPrototypeOf(iterator);\n// %IteratorPrototype%\nvar proto2 = Object.getPrototypeOf(proto1);\n\nreturn proto2.hasOwnProperty(Symbol.iterator) &&\n  !proto1    .hasOwnProperty(Symbol.iterator) &&\n  !iterator  .hasOwnProperty(Symbol.iterator) &&\n  iterator[Symbol.iterator]() === iterator;"
            }, Subtest { name : "Map[Symbol.species]", exec :
            "var prop = Object.getOwnPropertyDescriptor(Map, Symbol.species);\nreturn 'get' in prop && Map[Symbol.species] === Map;"
            },
        ]
    }
}
