use crate::feature::{Meta, Subtest};
use crate::features::Es6DestructuringParameters;
impl Meta for Es6DestructuringParameters {
    fn name(&self) -> &'static str {
        "destructuring, parameters"
    }
    fn key(&self) -> &'static str {
        "es6_destructuring_parameters"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-destructuring-assignment"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "with arrays", exec :
            "return function([a, , [b], c]) {\n  return a === 5 && b === 6 && c === void undefined;\n}([5, null, [6]]);"
            }, Subtest { name : "with sparse arrays", exec :
            "return function([a, , b]) {\n  return a === void undefined && b === void undefined;\n}([,,,]);"
            }, Subtest { name : "with strings", exec :
            "return function([a, b, c]) {\n  return a === \"a\" && b === \"b\" && c === void undefined;\n}(\"ab\");"
            }, Subtest { name : "with astral plane strings", exec :
            "return function([c]) {\n  return c === \"𠮷\";\n}(\"𠮷𠮶\");" },
            Subtest { name : "with generator instances", exec :
            "return function([a, b, c]) {\n  return a === 1 && b === 2 && c === void undefined;\n}(function*(){ yield 1; yield 2; }());"
            }, Subtest { name : "with generic iterables", exec :
            "return function([a, b, c]) {\n  return a === 1 && b === 2 && c === void undefined;\n}(global.__createIterableObject([1, 2]));"
            }, Subtest { name : "with instances of generic iterables", exec :
            "return function([a, b, c]) {\n  return a === 1 && b === 2 && c === void undefined;\n}(Object.create(global.__createIterableObject([1, 2])));"
            }, Subtest { name : "iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\n(function([a,b]) {}(iter));\nreturn closed;"
            }, Subtest { name : "trailing commas in iterable patterns", exec :
            "return function([a,]) {\n  return a === 1;\n}([1]);" }, Subtest { name :
            "with objects", exec :
            "return function({c, x:d, e}) {\n  return c === 7 && d === 8 && e === void undefined;\n}({c:7, x:8});"
            }, Subtest { name : "object destructuring with primitives", exec :
            "return function({toFixed}, {slice}) {\n  return toFixed === Number.prototype.toFixed\n    && slice === String.prototype.slice;\n}(2,'');"
            }, Subtest { name : "trailing commas in object patterns", exec :
            "return function({a,}) {\n  return a === 1;\n}({a:1});" }, Subtest { name :
            "throws on null and undefined", exec :
            "try {\n  (function({a}){}(null));\n  return false;\n} catch(e) {}\ntry {\n  (function({b}){}(undefined));\n  return false;\n} catch(e) {}\nreturn true;"
            }, Subtest { name : "computed properties", exec :
            "var qux = \"corge\";\nreturn function({ [qux]: grault }) {\n  return grault === \"garply\";\n}({ corge: \"garply\" });"
            }, Subtest { name : "nested", exec :
            "return function([e, {x:f, g}], {h, x:[i]}) {\n  return e === 9 && f === 10 && g === void undefined\n    && h === 11 && i === 12;\n}([9, {x:10}],{h:11, x:[12]});"
            }, Subtest { name : "'arguments' interaction", exec :
            "return (function({a, x:b, y:e}, [c, d]) {\n  return arguments[0].a === 1 && arguments[0].x === 2\n    && !(\"y\" in arguments[0]) && arguments[1] + '' === \"3,4\";\n}({a:1, x:2}, [3, 4]));"
            }, Subtest { name : "new Function() support", exec :
            "return new Function(\"{a, x:b, y:e}\",\"[c, d]\",\n  \"return a === 1 && b === 2 && c === 3 && \"\n  + \"d === 4 && e === void undefined;\"\n)({a:1, x:2}, [3, 4]);"
            }, Subtest { name : "in parameters, function 'length' property", exec :
            "return function({a, b}, [c, d]){}.length === 2;" }, Subtest { name : "rest",
            exec :
            "return function([a, ...b], [c, ...d]) {\n  return a === 3 && b instanceof Array && (b + \"\") === \"4,5\" &&\n     c === 6 && d instanceof Array && d.length === 0;\n}([3, 4, 5], [6]);"
            }, Subtest { name : "empty patterns", exec :
            "return function ([],{}){\n  return arguments[0] + '' === \"3,4\" && arguments[1].x === \"foo\";\n}([3,4],{x:\"foo\"});"
            }, Subtest { name : "defaults", exec :
            "return (function({a = 1, b = 0, c = 3, x:d = 0, y:e = 5},\n    [f = 6, g = 0, h = 8]) {\n  return a === 1 && b === 2 && c === 3 && d === 4 &&\n    e === 5 && f === 6 && g === 7 && h === 8;\n}({b:2, c:undefined, x:4},[, 7, undefined]));"
            }, Subtest { name : "defaults, separate scope", exec :
            "return (function({a=function(){\n  return typeof b === 'undefined';\n}}){\n  var b = 1;\n  return a();\n}({}));"
            }, Subtest { name : "defaults, new Function() support", exec :
            "return new Function(\"{a = 1, b = 0, c = 3, x:d = 0, y:e = 5}\",\n  \"return a === 1 && b === 2 && c === 3 && d === 4 && e === 5;\"\n)({b:2, c:undefined, x:4});"
            }, Subtest { name : "aliased defaults, arrow function", exec :
            "return ((a, {b: x = 0, c: y = 3}) => {\n  return a === 1 && x === 2 && y === 3;\n})(1, {b: 2});"
            }, Subtest { name : "shorthand defaults, arrow function", exec :
            "return ((a, {b = 0, c = 3}) => {\n  return a === 1 && b === 2 && c === 3;\n})(1, {b: 2});"
            }, Subtest { name : "duplicate identifier", exec :
            "try {\n  eval('var d = function d([d]) { return d };');\n  if (d([true]) !== true) return false;\n} catch (e) {\n  return !(e instanceof SyntaxError);\n}\n\ntry {\n  eval('var f = function f([id, id]) { return id }');\n  return false;\n} catch (e) {\n  return e instanceof SyntaxError;\n}"
            },
        ]
    }
}
