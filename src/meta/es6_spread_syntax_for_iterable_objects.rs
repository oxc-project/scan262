use crate::{
    feature::{Meta, Subtest},
    features::Es6SpreadSyntaxForIterableObjects,
};
impl Meta for Es6SpreadSyntaxForIterableObjects {
    fn name(&self) -> &'static str {
        "spread syntax for iterable objects"
    }

    fn key(&self) -> &'static str {
        "es6_spread_syntax_for_iterable_objects"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "syntax"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-argument-lists-runtime-semantics-argumentlistevaluation"
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_operator"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "with arrays, in function calls",
                exec: "return Math.max(...[1, 2, 3]) === 3",
            },
            Subtest {
                name: "with arrays, in array literals",
                exec: "return [...[1, 2, 3]][2] === 3;",
            },
            Subtest {
                name: "with sparse arrays, in function calls",
                exec: "var a = Array(...[,,]);\nreturn \"0\" in a && \"1\" in a && '' + a[0] + a[1] === \"undefinedundefined\";",
            },
            Subtest {
                name: "with sparse arrays, in array literals",
                exec: "var a = [...[,,]];\nreturn \"0\" in a && \"1\" in a && '' + a[0] + a[1] === \"undefinedundefined\";",
            },
            Subtest {
                name: "with strings, in function calls",
                exec: "return Math.max(...\"1234\") === 4;",
            },
            Subtest {
                name: "with strings, in array literals",
                exec: "return [\"a\", ...\"bcd\", \"e\"][3] === \"d\";",
            },
            Subtest {
                name: "with astral plane strings, in function calls",
                exec: "return Array(...\"𠮷𠮶\")[0] === \"𠮷\";",
            },
            Subtest {
                name: "with astral plane strings, in array literals",
                exec: "return [...\"𠮷𠮶\"][0] === \"𠮷\";",
            },
            Subtest {
                name: "with generator instances, in calls",
                exec: "var iterable = (function*(){ yield 1; yield 2; yield 3; }());\nreturn Math.max(...iterable) === 3;",
            },
            Subtest {
                name: "with generator instances, in arrays",
                exec: "var iterable = (function*(){ yield \"b\"; yield \"c\"; yield \"d\"; }());\nreturn [\"a\", ...iterable, \"e\"][3] === \"d\";",
            },
            Subtest {
                name: "with generic iterables, in calls",
                exec: "var iterable = global.__createIterableObject([1, 2, 3]);\nreturn Math.max(...iterable) === 3;",
            },
            Subtest {
                name: "with generic iterables, in arrays",
                exec: "var iterable = global.__createIterableObject([\"b\", \"c\", \"d\"]);\nreturn [\"a\", ...iterable, \"e\"][3] === \"d\";",
            },
            Subtest {
                name: "with instances of iterables, in calls",
                exec: "var iterable = global.__createIterableObject([1, 2, 3]);\nreturn Math.max(...Object.create(iterable)) === 3;",
            },
            Subtest {
                name: "with instances of iterables, in arrays",
                exec: "var iterable = global.__createIterableObject([\"b\", \"c\", \"d\"]);\nreturn [\"a\", ...Object.create(iterable), \"e\"][3] === \"d\";",
            },
            Subtest {
                name: "spreading non-iterables is a runtime error",
                exec: "try {\n  Math.max(...2);\n} catch(e) {\n  return Math.max(...[1, 2, 3]) === 3;\n}",
            },
        ]
    }
}
