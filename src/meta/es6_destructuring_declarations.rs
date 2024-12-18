use crate::feature::{Meta, Subtest};
use crate::features::Es6DestructuringDeclarations;
impl Meta for Es6DestructuringDeclarations {
    fn name(&self) -> &'static str {
        "destructuring, declarations"
    }
    fn key(&self) -> &'static str {
        "es6_destructuring_declarations"
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
            "var [a, , [b], c] = [5, null, [6]];\nreturn a === 5 && b === 6 && c === void undefined;",
            }, Subtest { name : "with sparse arrays", exec :
            "var [a, , b] = [,,,];\nreturn a === void undefined && b === void undefined;",
            }, Subtest { name : "with strings", exec :
            "var [a, b, c] = \"ab\";\nreturn a === \"a\" && b === \"b\" && c === void undefined;",
            }, Subtest { name : "with astral plane strings", exec :
            "var [c] = \"𠮷𠮶\";\nreturn c === \"𠮷\";", }, Subtest { name :
            "with generator instances", exec :
            "var [a, b, c] = (function*(){ yield 1; yield 2; }());\nreturn a === 1 && b === 2 && c === void undefined;",
            }, Subtest { name : "with generic iterables", exec :
            "var [a, b, c] = global.__createIterableObject([1, 2]);\nreturn a === 1 && b === 2 && c === void undefined;",
            }, Subtest { name : "with instances of generic iterables", exec :
            "var [a, b, c] = Object.create(global.__createIterableObject([1, 2]));\nreturn a === 1 && b === 2 && c === void undefined;",
            }, Subtest { name : "iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\nvar [a, b] = iter;\nreturn closed;",
            }, Subtest { name : "trailing commas in iterable patterns", exec :
            "var [a,] = [1];\nreturn a === 1;", }, Subtest { name : "with objects", exec
            :
            "var {c, x:d, e} = {c:7, x:8};\nreturn c === 7 && d === 8 && e === void undefined;",
            }, Subtest { name : "object destructuring with primitives", exec :
            "var {toFixed} = 2;\nvar {slice} = '';\nreturn toFixed === Number.prototype.toFixed\n  && slice === String.prototype.slice;",
            }, Subtest { name : "trailing commas in object patterns", exec :
            "var {a,} = {a:1};\nreturn a === 1;", }, Subtest { name :
            "throws on null and undefined", exec :
            "try {\n  var {a} = null;\n  return false;\n} catch(e) {\n  if (!(e instanceof TypeError))\n    return false;\n}\ntry {\n  var {b} = void undefined;\n  return false;\n} catch(e) {\n  if (!(e instanceof TypeError))\n    return false;\n}\nreturn true;",
            }, Subtest { name : "computed properties", exec :
            "var qux = \"corge\";\nvar { [qux]: grault } = { corge: \"garply\" };\nreturn grault === \"garply\";",
            }, Subtest { name : "multiples in a single var statement", exec :
            "var [a,b] = [5,6], {c,d} = {c:7,d:8};\nreturn a === 5 && b === 6 && c === 7 && d === 8;",
            }, Subtest { name : "nested", exec :
            "var [e, {x:f, g}] = [9, {x:10}];\nvar {h, x:[i]} = {h:11, x:[12]};\nreturn e === 9 && f === 10 && g === void undefined\n  && h === 11 && i === 12;",
            }, Subtest { name : "in for-in loop heads", exec :
            "for(var [i, j, k] in { qux: 1 }) {\n  return i === \"q\" && j === \"u\" && k === \"x\";\n}",
            }, Subtest { name : "in for-of loop heads", exec :
            "for(var [i, j, k] of [[1,2,3]]) {\n  return i === 1 && j === 2 && k === 3;\n}",
            }, Subtest { name : "in catch heads", exec :
            "try {\n  throw [1,2];\n} catch([i,j]) {\n  try {\n    throw { k: 3, l: 4 };\n  } catch({k, l}) {\n    return i === 1 && j === 2 && k === 3 && l === 4;\n  }\n}",
            }, Subtest { name : "rest", exec :
            "var [a, ...b] = [3, 4, 5];\nvar [c, ...d] = [6];\nreturn a === 3 && b instanceof Array && (b + \"\") === \"4,5\" &&\n   c === 6 && d instanceof Array && d.length === 0;",
            }, Subtest { name : "defaults", exec :
            "var {a = 1, b = 0, z:c = 3} = {b:2, z:undefined};\nvar [d = 0, e = 5, f = 6] = [4,,undefined];\nreturn a === 1 && b === 2 && c === 3\n  && d === 4 && e === 5 && f === 6;",
            }, Subtest { name : "defaults, let temporal dead zone", exec :
            "var {a, b = 2} = {a:1};\ntry {\n  eval(\"let {c = c} = {};\");\n  return false;\n} catch(e){}\ntry {\n  eval(\"let {c = d, d} = {d:1};\");\n  return false;\n} catch(e){}\nreturn a === 1 && b === 2;",
            },
        ]
    }
}
