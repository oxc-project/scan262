use crate::feature::{Meta, Subtest};
use crate::features::Es6WellKnownSymbols;
impl Meta for Es6WellKnownSymbols {
    fn name(&self) -> &'static str {
        "well-known symbols"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-well-known-symbols"
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
            Subtest { name : "Symbol.hasInstance", exec :
            "var passed = false;\nvar obj = { foo: true };\nvar C = function(){};\nObject.defineProperty(C, Symbol.hasInstance, {\n  value: function(inst) { passed = inst.foo; return false; }\n});\nobj instanceof C;\nreturn passed;",
            }, Subtest { name : "Symbol.isConcatSpreadable, non-spreadable array", exec :
            "var a = [], b = [];\nb[Symbol.isConcatSpreadable] = false;\na = a.concat(b);\nreturn a[0] === b;",
            }, Subtest { name :
            "Symbol.isConcatSpreadable, spreadable object with poisoned getter", exec :
            "if (typeof Symbol !== 'function' || !Symbol.isConcatSpreadable) {\n  return null;\n}\nvar spreadableHasPoisonedIndex = { length: Math.pow(2, 53) - 1 };\nspreadableHasPoisonedIndex[Symbol.isConcatSpreadable] = true;\nObject.defineProperty(spreadableHasPoisonedIndex, 0, {\n  get: function () { throw new SyntaxError(); }\n});\ntry {\n  [].concat(spreadableHasPoisonedIndex);\n  return false;\n} catch (e) {\n  return !!e && e.name === 'SyntaxError';\n}",
            }, Subtest { name : "Symbol.iterator, existence", exec :
            "return \"iterator\" in Symbol;", }, Subtest { name :
            "Symbol.iterator, arguments object", exec :
            "return (function() {\n  return typeof arguments[Symbol.iterator] === 'function'\n    && Object.hasOwnProperty.call(arguments, Symbol.iterator);\n}());",
            }, Subtest { name : "Symbol.species, existence", exec :
            "return \"species\" in Symbol;", }, Subtest { name :
            "Symbol.species, Array.prototype.concat", exec :
            "var obj = [];\nobj.constructor = {};\nobj.constructor[Symbol.species] = function() {\n    return { foo: 1 };\n};\nreturn Array.prototype.concat.call(obj, []).foo === 1;",
            }, Subtest { name : "Symbol.species, Array.prototype.filter", exec :
            "var obj = [];\nobj.constructor = {};\nobj.constructor[Symbol.species] = function() {\n    return { foo: 1 };\n};\nreturn Array.prototype.filter.call(obj, Boolean).foo === 1;",
            }, Subtest { name : "Symbol.species, Array.prototype.map", exec :
            "var obj = [];\nobj.constructor = {};\nobj.constructor[Symbol.species] = function() {\n    return { foo: 1 };\n};\nreturn Array.prototype.map.call(obj, Boolean).foo === 1;",
            }, Subtest { name : "Symbol.species, Array.prototype.slice", exec :
            "var obj = [];\nobj.constructor = {};\nobj.constructor[Symbol.species] = function() {\n    return { foo: 1 };\n};\nreturn Array.prototype.slice.call(obj, 0).foo === 1;",
            }, Subtest { name : "Symbol.species, Array.prototype.splice", exec :
            "var obj = [];\nobj.constructor = {};\nobj.constructor[Symbol.species] = function() {\n    return { foo: 1 };\n};\nreturn Array.prototype.splice.call(obj, 0).foo === 1;",
            }, Subtest { name : "Symbol.species, RegExp.prototype[Symbol.split]", exec :
            "var passed = false;\nvar obj = { constructor: {} };\nobj[Symbol.split] = RegExp.prototype[Symbol.split];\nobj.constructor[Symbol.species] = function() {\n  passed = true;\n  return /./;\n};\n\"\".split(obj);\nreturn passed;",
            }, Subtest { name : "Symbol.species, Promise.prototype.then", exec :
            "var promise      = new Promise(function(resolve){ resolve(42); });\nvar FakePromise1 = promise.constructor = function(exec){ exec(function(){}, function(){}); };\nvar FakePromise2 = function(exec){ exec(function(){}, function(){}); };\nObject.defineProperty(FakePromise1, Symbol.species, {value: FakePromise2});\nreturn promise.then(function(){}) instanceof FakePromise2;",
            }, Subtest { name : "Symbol.replace", exec :
            "var O = {};\nO[Symbol.replace] = function(){\n  return 42;\n};\nreturn ''.replace(O) === 42;",
            }, Subtest { name : "Symbol.search", exec :
            "var O = {};\nO[Symbol.search] = function(){\n  return 42;\n};\nreturn ''.search(O) === 42;",
            }, Subtest { name : "Symbol.split", exec :
            "var O = {};\nO[Symbol.split] = function(){\n  return 42;\n};\nreturn ''.split(O) === 42;",
            }, Subtest { name : "Symbol.match", exec :
            "var O = {};\nO[Symbol.match] = function(){\n  return 42;\n};\nreturn ''.match(O) === 42;",
            }, Subtest { name : "Symbol.match, RegExp constructor", exec :
            "var re = /./;\nre[Symbol.match] = false;\nvar foo = {constructor: RegExp};\nfoo[Symbol.match] = true;\nreturn RegExp(re) !== re && RegExp(foo) === foo;",
            }, Subtest { name : "Symbol.match, String.prototype.startsWith", exec :
            "var re = /./;\ntry {\n  '/./'.startsWith(re);\n} catch(e){\n  re[Symbol.match] = false;\n  return '/./'.startsWith(re);\n}",
            }, Subtest { name : "Symbol.match, String.prototype.endsWith", exec :
            "var re = /./;\ntry {\n  '/./'.endsWith(re);\n} catch(e){\n  re[Symbol.match] = false;\n  return '/./'.endsWith(re);\n}",
            }, Subtest { name : "Symbol.match, String.prototype.includes", exec :
            "var re = /./;\ntry {\n  '/./'.includes(re);\n} catch(e){\n  re[Symbol.match] = false;\n  return '/./'.includes(re);\n}",
            }, Subtest { name : "Symbol.toPrimitive", exec :
            "var a = {}, b = {}, c = {};\nvar passed = 0;\na[Symbol.toPrimitive] = function(hint) { passed += hint === \"number\";  return 0; };\nb[Symbol.toPrimitive] = function(hint) { passed += hint === \"string\";  return 0; };\nc[Symbol.toPrimitive] = function(hint) { passed += hint === \"default\"; return 0; };\n\na >= 0;\nb in {};\nc == 0; // eslint-disable-line eqeqeq\nreturn passed === 3;",
            }, Subtest { name : "Symbol.toStringTag", exec :
            "var a = {};\na[Symbol.toStringTag] = \"foo\";\nreturn (a + \"\") === \"[object foo]\";",
            }, Subtest { name : "Symbol.toStringTag affects existing built-ins", exec :
            "var s = Symbol.toStringTag;\nvar passed = true;\n[\n  [Array.prototype, []],\n  [String.prototype, ''],\n  [arguments, arguments],\n  [Function.prototype, function(){}],\n  [Error.prototype, new Error()],\n  [Boolean.prototype, true],\n  [Number.prototype, 2],\n  [Date.prototype, new Date()],\n  [RegExp.prototype, /./]\n].forEach(function(pair){\n  pair[0][s] = \"foo\";\n  passed &= (Object.prototype.toString.call(pair[1]) === \"[object foo]\");\n  delete pair[0][s];\n});\nreturn passed;",
            }, Subtest { name : "Symbol.toStringTag, new built-ins", exec :
            "var passed = true;\nvar s = Symbol.toStringTag;\n[\n  [String, \"String Iterator\"],\n  [Array, \"Array Iterator\"],\n  [Map, \"Map Iterator\"],\n  [Set, \"Set Iterator\"]\n].forEach(function(pair){\n  var iterProto = Object.getPrototypeOf(new pair[0]()[Symbol.iterator]());\n  passed = passed\n    && iterProto.hasOwnProperty(s)\n    && iterProto[s] === pair[1];\n});\npassed = passed\n  && Object.getPrototypeOf(function*(){})[s] === \"GeneratorFunction\"\n  && Object.getPrototypeOf(function*(){}())[s] === \"Generator\"\n  && Map.prototype[s] === \"Map\"\n  && Set.prototype[s] === \"Set\"\n  && ArrayBuffer.prototype[s] === \"ArrayBuffer\"\n  && DataView.prototype[s] === \"DataView\"\n  && Promise.prototype[s] === \"Promise\"\n  && Symbol.prototype[s] === \"Symbol\"\n  && typeof Object.getOwnPropertyDescriptor(\n    Object.getPrototypeOf(Int8Array).prototype, Symbol.toStringTag).get === \"function\";\n  return passed;",
            }, Subtest { name : "Symbol.toStringTag, misc. built-ins", exec :
            "var s = Symbol.toStringTag;\nreturn Math[s] === \"Math\"\n  && JSON[s] === \"JSON\";",
            }, Subtest { name : "Symbol.unscopables", exec :
            "var a = { foo: 1, bar: 2 };\na[Symbol.unscopables] = { bar: true };\nwith (a) {\n  return foo === 1 && typeof bar === \"undefined\";\n}",
            },
        ]
    }
}
