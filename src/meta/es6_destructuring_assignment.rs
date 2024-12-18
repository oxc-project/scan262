use crate::{
    feature::{Meta, Subtest},
    features::Es6DestructuringAssignment,
};
impl Meta for Es6DestructuringAssignment {
    fn name(&self) -> &'static str {
        "destructuring, assignment"
    }

    fn key(&self) -> &'static str {
        "es6_destructuring_assignment"
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
            Subtest {
                name: "with arrays",
                exec: "var a,b,c;\n[a, , [b], c] = [5, null, [6]];\nreturn a === 5 && b === 6 && c === void undefined;",
            },
            Subtest {
                name: "with sparse arrays",
                exec: "var a, b;\n[a, , b] = [,,,];\nreturn a === void undefined && b === void undefined;",
            },
            Subtest {
                name: "with strings",
                exec: "var a,b,c;\n[a, b, c] = \"ab\";\nreturn a === \"a\" && b === \"b\" && c === void undefined;",
            },
            Subtest {
                name: "with astral plane strings",
                exec: "var c;\n[c] = \"𠮷𠮶\";\nreturn c === \"𠮷\";",
            },
            Subtest {
                name: "with generator instances",
                exec: "var a,b,c;\n[a, b, c] = (function*(){ yield 1; yield 2; }());\nreturn a === 1 && b === 2 && c === void undefined;",
            },
            Subtest {
                name: "with generic iterables",
                exec: "var a,b,c;\n[a, b, c] = global.__createIterableObject([1, 2]);\nreturn a === 1 && b === 2 && c === void undefined;",
            },
            Subtest {
                name: "with instances of generic iterables",
                exec: "var a,b,c;\n[a, b, c] = Object.create(global.__createIterableObject([1, 2]));\nreturn a === 1 && b === 2 && c === void undefined;",
            },
            Subtest {
                name: "iterator closing",
                exec: "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\nvar a,b;\n[a, b] = iter;\nreturn closed;",
            },
            Subtest {
                name: "iterable destructuring expression",
                exec: "var a, b, iterable = [1,2];\nreturn ([a, b] = iterable) === iterable;",
            },
            Subtest {
                name: "chained iterable destructuring",
                exec: "var a,b,c,d;\n[a,b] = [c,d] = [1,2];\nreturn a === 1 && b === 2 && c === 1 && d === 2;",
            },
            Subtest {
                name: "trailing commas in iterable patterns",
                exec: "var a;\n[a,] = [1];\nreturn a === 1;",
            },
            Subtest {
                name: "with objects",
                exec: "var c,d,e;\n({c, x:d, e} = {c:7, x:8});\nreturn c === 7 && d === 8 && e === void undefined;",
            },
            Subtest {
                name: "object destructuring with primitives",
                exec: "var toFixed, slice;\n({toFixed} = 2);\n({slice} = '');\nreturn toFixed === Number.prototype.toFixed\n  && slice === String.prototype.slice;",
            },
            Subtest {
                name: "trailing commas in object patterns",
                exec: "var a;\n({a,} = {a:1});\nreturn a === 1;",
            },
            Subtest {
                name: "object destructuring expression",
                exec: "var a, b, obj = { a:1, b:2 };\nreturn ({a,b} = obj) === obj;",
            },
            Subtest {
                name: "parenthesised left-hand-side is a syntax error",
                exec: "var a, b;\n({a,b} = {a:1,b:2});\ntry {\n  eval(\"({a,b}) = {a:3,b:4};\");\n}\ncatch(e) {\n  return a === 1 && b === 2;\n}",
            },
            Subtest {
                name: "chained object destructuring",
                exec: "var a,b,c,d;\n({a,b} = {c,d} = {a:1,b:2,c:3,d:4});\nreturn a === 1 && b === 2 && c === 3 && d === 4;",
            },
            Subtest {
                name: "throws on null and undefined",
                exec: "var a,b;\ntry {\n  ({a} = null);\n  return false;\n} catch(e) {\n  if (!(e instanceof TypeError))\n    return false;\n}\ntry {\n  ({b} = void undefined);\n  return false;\n} catch(e) {\n  if (!(e instanceof TypeError))\n    return false;\n}\nreturn true;",
            },
            Subtest {
                name: "computed properties",
                exec: "var grault, qux = \"corge\";\n({ [qux]: grault } = { corge: \"garply\" });\nreturn grault === \"garply\";",
            },
            Subtest {
                name: "nested",
                exec: "var e,f,g,h,i;\n[e, {x:f, g}] = [9, {x:10}];\n({h, x:[i]} = {h:11, x:[12]});\nreturn e === 9 && f === 10 && g === void undefined\n  && h === 11 && i === 12;",
            },
            Subtest {
                name: "rest",
                exec: "var a,b,c,d;\n[a, ...b] = [3, 4, 5];\n[c, ...d] = [6];\nreturn a === 3 && b instanceof Array && (b + \"\") === \"4,5\" &&\n   c === 6 && d instanceof Array && d.length === 0;",
            },
            Subtest {
                name: "nested rest",
                exec: "var a = [1, 2, 3], first, last;\n[first, ...[a[2], last]] = a;\nreturn first === 1 && last === 3 && (a + \"\") === \"1,2,2\";",
            },
            Subtest {
                name: "empty patterns",
                exec: "[] = [1,2];\n({} = {a:1,b:2});\nreturn true;",
            },
            Subtest {
                name: "defaults",
                exec: "var a,b,c,d,e,f;\n({a = 1, b = 0, z:c = 3} = {b:2, z:undefined});\n[d = 0, e = 5, f = 6] = [4,,undefined];\nreturn a === 1 && b === 2 && c === 3\n  && d === 4 && e === 5 && f === 6;",
            },
        ]
    }
}
