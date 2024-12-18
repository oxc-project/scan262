use crate::feature::{Meta, Subtest};
use crate::features::Es6OwnPropertyOrder;
impl Meta for Es6OwnPropertyOrder {
    fn name(&self) -> &'static str {
        "own property order"
    }
    fn key(&self) -> &'static str {
        "es6_own_property_order"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-ordinary-object-internal-methods-and-internal-slots-ownpropertykeys"
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
            Subtest { name : "Object.keys", exec :
            "var obj = {\n  // Non-negative integer names appear first in value order\n  2: true,\n  0: true,\n  1: true,\n  // Other string names appear in source order\n  ' ': true,\n  // Non-negative integers are sorted above other names\n  9: true,\n  D: true,\n  B: true,\n  // Negative integers are treated as other names\n  '-1': true\n};\n// Other string names are added in order of creation\nobj.A = true;\n// Non-negative integer names, conversely, ignore order of creation\nobj[3] = true;\n// Having a total of 20+ properties doesn't affect property order\n\"EFGHIJKLMNOPQRSTUVWXYZ\".split('').forEach(function(key){\n  obj[key] = true;\n});\n// Object.defineProperty doesn't affect the above rules\nObject.defineProperty(obj, 'C', { value: true, enumerable: true });\nObject.defineProperty(obj, '4', { value: true, enumerable: true });\n// Deleting and reinserting a property doesn't preserve its position\ndelete obj[2];\nobj[2] = true;\n\nvar forInOrder = '';\nfor(var key in obj)forInOrder += key;\n\nreturn Object.keys(obj).join('') === forInOrder;",
            }, Subtest { name : "Object.getOwnPropertyNames", exec :
            "var obj = {\n  2: true,\n  0: true,\n  1: true,\n  ' ': true,\n  9: true,\n  D: true,\n  B: true,\n  '-1': true\n};\nobj.A = true;\nobj[3] = true;\n\"EFGHIJKLMNOPQRSTUVWXYZ\".split('').forEach(function(key){\n  obj[key] = true;\n});\nObject.defineProperty(obj, 'C', { value: true, enumerable: true });\nObject.defineProperty(obj, '4', { value: true, enumerable: true });\ndelete obj[2];\nobj[2] = true;\n\nreturn Object.getOwnPropertyNames(obj).join('') === \"012349 DB-1AEFGHIJKLMNOPQRSTUVWXYZC\";",
            }, Subtest { name : "Object.assign", exec :
            "var result = '';\nvar target = {};\n\n\"012349 DBACEFGHIJKLMNOPQRST\".split('').concat(-1).forEach(function(key){\n  Object.defineProperty(target, key, {\n    set: function(){\n      result += key;\n    }\n  })\n});\n\nvar obj = {2: 2, 0: 0, 1: 1, ' ': ' ', 9: 9, D: 'D', B: 'B', '-1': '-1'};\nObject.defineProperty(obj, 'A', {value: 'A',  enumerable: true});\nObject.defineProperty(obj, '3', {value: '3',  enumerable: true});\nObject.defineProperty(obj, 'C', {value: 'C',  enumerable: true});\nObject.defineProperty(obj, '4', {value: '4',  enumerable: true});\ndelete obj[2];\nobj[2] = true;\n\n\"EFGHIJKLMNOPQRST\".split('').forEach(function(key){\n  obj[key] = key;\n});\n\nObject.assign(target, obj);\n\nreturn result === \"012349 DB-1ACEFGHIJKLMNOPQRST\";",
            }, Subtest { name : "JSON.stringify", exec :
            "var obj = {\n  2: true,\n  0: true,\n  1: true,\n  ' ': true,\n  9: true,\n  D: true,\n  B: true,\n  '-1': true\n};\nobj.A = true;\nobj[3] = true;\n\"EFGHIJKLMNOPQRSTUVWXYZ\".split('').forEach(function(key){\n  obj[key] = true;\n});\nObject.defineProperty(obj, 'C', { value: true, enumerable: true });\nObject.defineProperty(obj, '4', { value: true, enumerable: true });\ndelete obj[2];\nobj[2] = true;\n\nreturn JSON.stringify(obj) ===\n  '{\"0\":true,\"1\":true,\"2\":true,\"3\":true,\"4\":true,\"9\":true,\" \":true,\"D\":true,\"B\":true,\"-1\":true,\"A\":true,\"E\":true,\"F\":true,\"G\":true,\"H\":true,\"I\":true,\"J\":true,\"K\":true,\"L\":true,\"M\":true,\"N\":true,\"O\":true,\"P\":true,\"Q\":true,\"R\":true,\"S\":true,\"T\":true,\"U\":true,\"V\":true,\"W\":true,\"X\":true,\"Y\":true,\"Z\":true,\"C\":true}';",
            }, Subtest { name : "JSON.parse", exec :
            "var result = '';\nJSON.parse(\n  '{\"0\":true,\"1\":true,\"2\":true,\"3\":true,\"4\":true,\"9\":true,\" \":true,\"D\":true,\"B\":true,\"-1\":true,\"E\":true,\"F\":true,\"G\":true,\"H\":true,\"I\":true,\"J\":true,\"K\":true,\"L\":true,\"A\":true,\"C\":true}',\n  function reviver(k,v) {\n    result += k;\n    return v;\n  }\n);\nreturn result === \"012349 DB-1EFGHIJKLAC\";",
            }, Subtest { name : "Reflect.ownKeys, string key order", exec :
            "var obj = {\n  2: true,\n  0: true,\n  1: true,\n  ' ': true,\n  9: true,\n  D: true,\n  B: true,\n  '-1': true\n};\nobj.A = true;\nobj[3] = true;\n\"EFGHIJKLMNOPQRSTUVWXYZ\".split('').forEach(function(key){\n  obj[key] = true;\n});\nObject.defineProperty(obj, 'C', { value: true, enumerable: true });\nObject.defineProperty(obj, '4', { value: true, enumerable: true });\ndelete obj[2];\nobj[2] = true;\n\nreturn Reflect.ownKeys(obj).join('') === \"012349 DB-1AEFGHIJKLMNOPQRSTUVWXYZC\";",
            }, Subtest { name : "Reflect.ownKeys, symbol key order", exec :
            "var sym1 = Symbol(), sym2 = Symbol(), sym3 = Symbol();\nvar obj = {\n  1: true,\n  A: true\n};\nobj.B = true;\nobj[sym1] = true;\nobj[2] = true;\nobj[sym2] = true;\nObject.defineProperty(obj, 'C', { value: true, enumerable: true });\nObject.defineProperty(obj, sym3,{ value: true, enumerable: true });\nObject.defineProperty(obj, 'D', { value: true, enumerable: true });\n\nvar result = Reflect.ownKeys(obj);\nvar l = result.length;\nreturn result[l-3] === sym1 && result[l-2] === sym2 && result[l-1] === sym3;",
            },
        ]
    }
}
