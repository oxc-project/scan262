use crate::feature::{Meta, Subtest};
use crate::features::Es6ArrayStaticMethods;
impl Meta for Es6ArrayStaticMethods {
    fn name(&self) -> &'static str {
        "Array static methods"
    }
    fn key(&self) -> &'static str {
        "es6_array_static_methods"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-in extensions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-array-constructor"
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
            Subtest { name : "Array.from, array-like objects", exec :
            "return Array.from({ 0: \"foo\", 1: \"bar\", length: 2 }) + '' === \"foo,bar\";"
            }, Subtest { name : "Array.from, generator instances", exec :
            "var iterable = (function*(){ yield 1; yield 2; yield 3; }());\nreturn Array.from(iterable) + '' === \"1,2,3\";"
            }, Subtest { name : "Array.from, generic iterables", exec :
            "var iterable = global.__createIterableObject([1, 2, 3]);\nreturn Array.from(iterable) + '' === \"1,2,3\";"
            }, Subtest { name : "Array.from, instances of generic iterables", exec :
            "var iterable = global.__createIterableObject([1, 2, 3]);\nreturn Array.from(Object.create(iterable)) + '' === \"1,2,3\";"
            }, Subtest { name : "Array.from map function, array-like objects", exec :
            "return Array.from({ 0: \"foo\", 1: \"bar\", length: 2 }, function(e, i) {\n  return e + this.baz + i;\n}, { baz: \"d\" }) + '' === \"food0,bard1\";"
            }, Subtest { name : "Array.from map function, generator instances", exec :
            "var iterable = (function*(){ yield \"foo\"; yield \"bar\"; yield \"bal\"; }());\nreturn Array.from(iterable, function(e, i) {\n  return e + this.baz + i;\n}, { baz: \"d\" }) + '' === \"food0,bard1,bald2\";"
            }, Subtest { name : "Array.from map function, generic iterables", exec :
            "var iterable = global.__createIterableObject([\"foo\", \"bar\", \"bal\"]);\nreturn Array.from(iterable, function(e, i) {\n  return e + this.baz + i;\n}, { baz: \"d\" }) + '' === \"food0,bard1,bald2\";"
            }, Subtest { name : "Array.from map function, instances of iterables", exec :
            "var iterable = global.__createIterableObject([\"foo\", \"bar\", \"bal\"]);\nreturn Array.from(Object.create(iterable), function(e, i) {\n  return e + this.baz + i;\n}, { baz: \"d\" }) + '' === \"food0,bard1,bald2\";"
            }, Subtest { name : "Array.from, iterator closing", exec :
            "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'return': function(){ closed = true; return {}; }\n});\ntry {\n  Array.from(iter, function() { throw 42 });\n} catch(e){}\nreturn closed;"
            }, Subtest { name : "Array.of", exec :
            "return typeof Array.of === 'function' &&\n  Array.of(2)[0] === 2;" },
            Subtest { name : "Array[Symbol.species]", exec :
            "var prop = Object.getOwnPropertyDescriptor(Array, Symbol.species);\nreturn 'get' in prop && Array[Symbol.species] === Array;"
            },
        ]
    }
}
