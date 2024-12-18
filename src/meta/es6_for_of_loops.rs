use crate::feature::{Meta, Subtest};
use crate::features::Es6ForOfLoops;
impl Meta for Es6ForOfLoops {
    fn name(&self) -> &'static str {
        "for..of loops"
    }
    fn key(&self) -> &'static str {
        "es6_for_of_loops"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-for-in-and-for-of-statements"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "with arrays", exec :
            "var arr = [5];\nfor (var item of arr)\n  return item === 5;", }, Subtest {
            name : "with sparse arrays", exec :
            "var arr = [,,];\nvar count = 0;\nfor (var item of arr)\n  count += (item === void undefined);\nreturn count === 2;",
            }, Subtest { name : "with strings", exec :
            "var str = \"\";\nfor (var item of \"foo\")\n  str += item;\nreturn str === \"foo\";",
            }, Subtest { name : "with astral plane strings", exec :
            "var str = \"\";\nfor (var item of \"𠮷𠮶\")\n  str += item + \" \";\nreturn str === \"𠮷 𠮶 \";",
            }, Subtest { name : "with generator instances", exec :
            "var result = \"\";\nvar iterable = (function*(){ yield 1; yield 2; yield 3; }());\nfor (var item of iterable) {\n  result += item;\n}\nreturn result === \"123\";",
            }, Subtest { name : "with generic iterables", exec :
            "var result = \"\";\nvar iterable = global.__createIterableObject([1, 2, 3]);\nfor (var item of iterable) {\n  result += item;\n}\nreturn result === \"123\";",
            }, Subtest { name : "with instances of generic iterables", exec :
            "var result = \"\";\nvar iterable = global.__createIterableObject([1, 2, 3]);\nfor (var item of Object.create(iterable)) {\n  result += item;\n}\nreturn result === \"123\";",
            }, Subtest { name : "iterator closing, break", exec :
            "var closed = false;\nvar iter = __createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\nfor (var it of iter) break;\nreturn closed;",
            }, Subtest { name : "iterator closing, throw", exec :
            "var closed = false;\nvar iter = __createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\ntry {\n  for (var it of iter) throw 0;\n} catch(e){}\nreturn closed;",
            },
        ]
    }
}
