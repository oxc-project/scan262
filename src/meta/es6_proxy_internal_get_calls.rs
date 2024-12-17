use crate::feature::{Meta, Subtest};
use crate::features::Es6ProxyInternalGetCalls;
impl Meta for Es6ProxyInternalGetCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'get' calls"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-proxy-object-internal-methods-and-internal-slots"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/get"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "ToPrimitive", exec :
            "// ToPrimitive -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({toString:Function()}, { get: function(o, k) { get.push(k); return o[k]; }});\np + 3;\nreturn get[0] === Symbol.toPrimitive && get.slice(1) + '' === \"valueOf,toString\";",
            }, Subtest { name : "CreateListFromArrayLike", exec :
            "// CreateListFromArrayLike -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({length:2, 0:0, 1:0}, { get: function(o, k) { get.push(k); return o[k]; }});\nFunction.prototype.apply({}, p);\nreturn get + '' === \"length,0,1\";",
            }, Subtest { name : "instanceof operator", exec :
            "// InstanceofOperator -> GetMethod -> GetV -> [[Get]]\n// InstanceofOperator -> OrdinaryHasInstance -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy(Function(), { get: function(o, k) { get.push(k); return o[k]; }});\n({}) instanceof p;\nreturn get[0] === Symbol.hasInstance && get.slice(1) + '' === \"prototype\";",
            }, Subtest { name : "HasBinding", exec :
            "// HasBinding -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({foo:1}, { get: function(o, k) { get.push(k); return o[k]; }});\np[Symbol.unscopables] = p;\nwith(p) {\n  typeof foo;\n}\nreturn get[0] === Symbol.unscopables && get.slice(1) + '' === \"foo\";",
            }, Subtest { name : "CreateDynamicFunction", exec :
            "// CreateDynamicFunction -> GetPrototypeFromConstructor -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy(Function, { get: function(o, k) { get.push(k); return o[k]; }});\nnew p;\nreturn get + '' === \"prototype\";",
            }, Subtest { name : "ClassDefinitionEvaluation", exec :
            "// ClassDefinitionEvaluation -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy(Function(), { get: function(o, k) { get.push(k); return o[k]; }});\nclass C extends p {}\nreturn get + '' === \"prototype\";",
            }, Subtest { name : "IteratorComplete, IteratorValue", exec :
            "// IteratorComplete -> Get -> [[Get]]\n// IteratorValue -> Get -> [[Get]]\nvar get = [];\nvar iterable = {};\niterable[Symbol.iterator] = function() {\n  return {\n    next: function() {\n      return new Proxy({ value: 2, done: false }, { get: function(o, k) { get.push(k); return o[k]; }});\n    }\n  };\n}\nvar i = 0;\nfor(var e of iterable) {\n  if (++i >= 2) break;\n}\nreturn get + '' === \"done,value,done,value\";",
            }, Subtest { name : "ToPropertyDescriptor", exec :
            "// ToPropertyDescriptor -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({\n    enumerable: true, configurable: true, value: true,\n    writable: true, get: Function(), set: Function()\n  }, { get: function(o, k) { get.push(k); return o[k]; }});\ntry {\n  // This will throw, since it will have true for both \"get\" and \"value\",\n  // but not before performing a Get on every property.\n  Object.defineProperty({}, \"foo\", p);\n} catch(e) {\n  return get + '' === \"enumerable,configurable,value,writable,get,set\";\n}",
            }, Subtest { name : "Object.assign", exec :
            "// Object.assign -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({foo:1, bar:2}, { get: function(o, k) { get.push(k); return o[k]; }});\nObject.assign({}, p);\nreturn get + '' === \"foo,bar\";",
            }, Subtest { name : "Object.defineProperties", exec :
            "// Object.defineProperties -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({foo:{}, bar:{}}, { get: function(o, k) { get.push(k); return o[k]; }});\nObject.defineProperties({}, p);\nreturn get + '' === \"foo,bar\";",
            }, Subtest { name : "Function.prototype.bind", exec :
            "// Function.prototype.bind -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy(Function(), { get: function(o, k) { get.push(k); return o[k]; }});\nFunction.prototype.bind.call(p);\nreturn get + '' === \"length,name\";",
            }, Subtest { name : "Error.prototype.toString", exec :
            "// Error.prototype.toString -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({}, { get: function(o, k) { get.push(k); return o[k]; }});\nError.prototype.toString.call(p);\nreturn get + '' === \"name,message\";",
            }, Subtest { name : "String.raw", exec :
            "// String.raw -> Get -> [[Get]]\nvar get = [];\nvar raw = new Proxy({length: 2, 0: '', 1: ''}, { get: function(o, k) { get.push(k); return o[k]; }});\nvar p = new Proxy({raw: raw}, { get: function(o, k) { get.push(k); return o[k]; }});\nString.raw(p);\nreturn get + '' === \"raw,length,0,1\";",
            }, Subtest { name : "RegExp constructor", exec :
            "// RegExp -> Get -> [[Get]]\nvar get = [];\nvar re = { constructor: null };\nre[Symbol.match] = true;\nvar p = new Proxy(re, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp(p);\nreturn get[0] === Symbol.match && get.slice(1) + '' === \"constructor,source,flags\";",
            }, Subtest { name : "RegExp.prototype.flags", exec :
            "// RegExp.prototype.flags -> Get -> [[Get]]\nvar expected = [];\n// Sorted alphabetically by shortname – \"gimsuy\".\nif ('hasIndices' in RegExp.prototype) expected.push('hasIndices');\nif ('global' in RegExp.prototype) expected.push('global');\nif ('ignoreCase' in RegExp.prototype) expected.push('ignoreCase');\nif ('multiline' in RegExp.prototype) expected.push('multiline');\nif ('dotAll' in RegExp.prototype) expected.push('dotAll');\nif ('unicode' in RegExp.prototype) expected.push('unicode');\nif ('unicodeSets' in RegExp.prototype) expected.push('unicodeSets');\nif ('sticky' in RegExp.prototype) expected.push('sticky');\n\nvar actual = [];\nvar p = new Proxy({}, { get: function(o, k) { actual.push(k); return o[k]; }});\nObject.getOwnPropertyDescriptor(RegExp.prototype, 'flags').get.call(p);\nif (expected.length !== actual.length) return false;\nfor (var i = 0; i < expected.length; i++) {\n  if (expected[i] !== actual[i]) return false;\n}\nreturn true;",
            }, Subtest { name : "RegExp.prototype.test", exec :
            "// RegExp.prototype.test -> RegExpExec -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({ exec: function() { return null; } }, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype.test.call(p);\nreturn get + '' === \"exec\";",
            }, Subtest { name : "RegExp.prototype.toString", exec :
            "// RegExp.prototype.toString -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({}, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype.toString.call(p);\nreturn get + '' === \"source,flags\";",
            }, Subtest { name : "RegExp.prototype[Symbol.match]", exec :
            "// RegExp.prototype[Symbol.match] -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({ exec: function() { return null; } }, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype[Symbol.match].call(p);\np.global = true;\nRegExp.prototype[Symbol.match].call(p);\nvar str = get + '';\nreturn str === \"global,exec,global,unicode,exec\" || str === 'flags,exec,flags,exec';",
            }, Subtest { name : "RegExp.prototype[Symbol.replace]", exec :
            "// RegExp.prototype[Symbol.replace] -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({ exec: function() { return null; } }, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype[Symbol.replace].call(p);\np.global = true;\nRegExp.prototype[Symbol.replace].call(p);\nvar str = get + '';\nreturn str === \"global,exec,global,unicode,exec\" || str === 'flags,exec,flags,exec';",
            }, Subtest { name : "RegExp.prototype[Symbol.search]", exec :
            "// RegExp.prototype[Symbol.search] -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({ exec: function() { return null; } }, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype[Symbol.search].call(p);\nreturn get + '' === \"lastIndex,exec,lastIndex\";",
            }, Subtest { name : "RegExp.prototype[Symbol.split]", exec :
            "// RegExp.prototype[Symbol.split] -> Get -> [[Get]]\nvar get = [];\nvar constructor = Function();\nconstructor[Symbol.species] = Object;\nvar p = new Proxy({ constructor: constructor, flags: '', exec: function() { return null; } }, { get: function(o, k) { get.push(k); return o[k]; }});\nRegExp.prototype[Symbol.split].call(p, \"\");\nreturn get + '' === \"constructor,flags,exec\";",
            }, Subtest { name : "Array.from", exec :
            "// Array.from -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({length: 2, 0: '', 1: ''}, { get: function(o, k) { get.push(k); return o[k]; }});\nArray.from(p);\nreturn get[0] === Symbol.iterator && get.slice(1) + '' === \"length,0,1\";",
            }, Subtest { name : "Array.prototype.concat", exec :
            "// Array.prototype.concat -> Get -> [[Get]]\nvar get = [];\nvar arr = [1];\narr.constructor = void undefined;\nvar p = new Proxy(arr, { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.concat.call(p,p);\nreturn get[0] === \"constructor\"\n  && get[1] === Symbol.isConcatSpreadable\n  && get[2] === \"length\"\n  && get[3] === \"0\"\n  && get[4] === get[1] && get[5] === get[2] && get[6] === get[3]\n  && get.length === 7;",
            }, Subtest { name : "Array.prototype iteration methods", exec :
            "// Array.prototype methods -> Get -> [[Get]]\nvar methods = ['copyWithin', 'every', 'fill', 'filter', 'find', 'findIndex', 'forEach',\n  'indexOf', 'join', 'lastIndexOf', 'map', 'reduce', 'reduceRight', 'some'];\nvar get;\nvar p = new Proxy({length: 2, 0: '', 1: ''}, { get: function(o, k) { get.push(k); return o[k]; }});\nfor(var i = 0; i < methods.length; i+=1) {\n  get = [];\n  Array.prototype[methods[i]].call(p, Function());\n  if (get + '' !== (\n    methods[i] === 'fill' ? \"length\" :\n    methods[i] === 'every' ? \"length,0\" :\n    methods[i] === 'lastIndexOf' || methods[i] === 'reduceRight' ? \"length,1,0\" :\n    \"length,0,1\"\n  )) {\n    return false;\n  }\n}\nreturn true;",
            }, Subtest { name : "Array.prototype.pop", exec :
            "// Array.prototype.pop -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy([0,1,2,3], { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.pop.call(p);\nreturn get + '' === \"length,3\";",
            }, Subtest { name : "Array.prototype.reverse", exec :
            "// Array.prototype.reverse -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy([0,,2,,4,,], { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.reverse.call(p);\nreturn get + '' === \"length,0,4,2\";",
            }, Subtest { name : "Array.prototype.shift", exec :
            "// Array.prototype.shift -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy([0,1,2,3], { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.shift.call(p);\nreturn get + '' === \"length,0,1,2,3\";",
            }, Subtest { name : "Array.prototype.splice", exec :
            "// Array.prototype.splice -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy([0,1,2,3], { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.splice.call(p,1,1);\nArray.prototype.splice.call(p,1,0,1);\nreturn get + '' === \"length,constructor,1,2,3,length,constructor,2,1\";",
            }, Subtest { name : "Array.prototype.toString", exec :
            "// Array.prototype.toString -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({ join:Function() }, { get: function(o, k) { get.push(k); return o[k]; }});\nArray.prototype.toString.call(p);\nreturn get + '' === \"join\";",
            }, Subtest { name : "JSON.stringify", exec :
            "// JSON.stringify -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({}, { get: function(o, k) { get.push(k); return o[k]; }});\nJSON.stringify(p);\nreturn get + '' === \"toJSON\";",
            }, Subtest { name : "Promise resolve functions", exec :
            "// Promise resolve functions -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({}, { get: function(o, k) { get.push(k); return o[k]; }});\nnew Promise(function(resolve){ resolve(p); });\nreturn get + '' === \"then\";",
            }, Subtest { name : "String.prototype.match", exec :
            "// String.prototype.match -> Get -> [[Get]]\nvar get = [];\nvar proxied = {};\nproxied[Symbol.toPrimitive] = Function();\nvar p = new Proxy(proxied, { get: function(o, k) { get.push(k); return o[k]; }});\n\"\".match(p);\nreturn get[0] === Symbol.match && get[1] === Symbol.toPrimitive && get.length === 2;",
            }, Subtest { name : "String.prototype.replace", exec :
            "// String.prototype.replace functions -> Get -> [[Get]]\nvar get = [];\nvar proxied = {};\nproxied[Symbol.toPrimitive] = Function();\nvar p = new Proxy(proxied, { get: function(o, k) { get.push(k); return o[k]; }});\n\"\".replace(p);\nreturn get[0] === Symbol.replace && get[1] === Symbol.toPrimitive && get.length === 2;",
            }, Subtest { name : "String.prototype.search", exec :
            "// String.prototype.search functions -> Get -> [[Get]]\nvar get = [];\nvar proxied = {};\nproxied[Symbol.toPrimitive] = Function();\nvar p = new Proxy(proxied, { get: function(o, k) { get.push(k); return o[k]; }});\n\"\".search(p);\nreturn get[0] === Symbol.search && get[1] === Symbol.toPrimitive && get.length === 2;",
            }, Subtest { name : "String.prototype.split", exec :
            "// String.prototype.split functions -> Get -> [[Get]]\nvar get = [];\nvar proxied = {};\nproxied[Symbol.toPrimitive] = Function();\nvar p = new Proxy(proxied, { get: function(o, k) { get.push(k); return o[k]; }});\n\"\".split(p);\nreturn get[0] === Symbol.split && get[1] === Symbol.toPrimitive && get.length === 2;",
            }, Subtest { name : "Date.prototype.toJSON", exec :
            "// Date.prototype.toJSON -> ToPrimitive -> Get -> [[Get]]\n// Date.prototype.toJSON -> Invoke -> GetMethod -> GetV -> [[Get]]\nvar get = [];\nvar p = new Proxy({toString:Function(),toISOString:Function()}, { get: function(o, k) { get.push(k); return o[k]; }});\nDate.prototype.toJSON.call(p);\nreturn get[0] === Symbol.toPrimitive && get.slice(1) + '' === \"valueOf,toString,toISOString\";",
            },
        ]
    }
}
