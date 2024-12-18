use crate::feature::{Meta, Subtest};
use crate::features::Es6ArrayPrototypeMethods;
impl Meta for Es6ArrayPrototypeMethods {
    fn name(&self) -> &'static str {
        "Array.prototype methods"
    }
    fn key(&self) -> &'static str {
        "es6_array_prototype_methods"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-in extensions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-array-prototype-object"
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
            Subtest { name : "Array.prototype.copyWithin", exec :
            "return typeof Array.prototype.copyWithin === 'function';", }, Subtest { name
            : "Array.prototype.find", exec :
            "return typeof Array.prototype.find === 'function';", }, Subtest { name :
            "Array.prototype.findIndex", exec :
            "return typeof Array.prototype.findIndex === 'function';", }, Subtest { name
            : "Array.prototype.fill", exec :
            "return typeof Array.prototype.fill === 'function';", }, Subtest { name :
            "Array.prototype.keys", exec :
            "return typeof Array.prototype.keys === 'function';", }, Subtest { name :
            "Array.prototype.values", exec :
            "return typeof Array.prototype.values === 'function';", }, Subtest { name :
            "Array.prototype.entries", exec :
            "return typeof Array.prototype.entries === 'function';", }, Subtest { name :
            "Array.prototype.splice", exec :
            "if ([0, 1, 2].splice(0).length !== 3) {\n  // IE <= 8 and other pre-ES6 engines fail this check\n  return false;\n}\n\nvar a = [1, 2];\nvar result = a.splice();\nif (a.length !== 2 || result.length !== 0) {\n  // Safari 5.0 has this bug\n  return false;\n}\n\nvar obj = {};\nArray.prototype.splice.call(obj, 0, 0, 1);\nif (obj.length !== 1) {\n  return false;\n}\n\nvar spliceWorksWithLargeSparseArrays = (function () {\n  // Per https://github.com/es-shims/es5-shim/issues/295\n  // Safari 7/8 breaks with sparse arrays of size 1e5 or greater\n  var arr = new Array(1e5);\n  // note: the index MUST be 8 or larger or the test will false pass\n  arr[8] = 'x';\n  arr.splice(1, 1);\n  for (var i = 0; i < arr.length; i += 1) {\n    if (arr[i] === 'x') {\n      return i === 7;\n    }\n  }\n  return false;\n}());\nvar spliceWorksWithSmallSparseArrays = (function () {\n  // Per https://github.com/es-shims/es5-shim/issues/295\n  // Opera 12.15 breaks on this, no idea why.\n  var n = 256;\n  var arr = [];\n  arr[n] = 'a';\n  arr.splice(n + 1, 0, 'b');\n  return arr[n] === 'a';\n}());\n\nreturn spliceWorksWithLargeSparseArrays && spliceWorksWithSmallSparseArrays;",
            }, Subtest { name : "Array.prototype[Symbol.iterator]", exec :
            "return typeof Array.prototype[Symbol.iterator] === 'function';", }, Subtest
            { name : "Array iterator prototype chain", exec :
            "// Iterator instance\nvar iterator = [][Symbol.iterator]();\n// %ArrayIteratorPrototype%\nvar proto1 = Object.getPrototypeOf(iterator);\n// %IteratorPrototype%\nvar proto2 = Object.getPrototypeOf(proto1);\n\nreturn proto2.hasOwnProperty(Symbol.iterator) &&\n  !proto1    .hasOwnProperty(Symbol.iterator) &&\n  !iterator  .hasOwnProperty(Symbol.iterator) &&\n  iterator[Symbol.iterator]() === iterator;",
            }, Subtest { name : "Array.prototype[Symbol.unscopables]", exec :
            "var unscopables = Array.prototype[Symbol.unscopables];\nif (!unscopables) {\n  return false;\n}\nvar ns = \"find,findIndex,fill,copyWithin,entries,keys,values\".split(\",\");\nfor (var i = 0; i < ns.length; i++) {\n  if (Array.prototype[ns[i]] && !unscopables[ns[i]]) return false;\n}\nreturn true;",
            },
        ]
    }
}
