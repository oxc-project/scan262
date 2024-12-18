use crate::{
    feature::{Meta, Subtest},
    features::Es6Generators,
};
impl Meta for Es6Generators {
    fn name(&self) -> &'static str {
        "generators"
    }

    fn key(&self) -> &'static str {
        "es6_generators"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "functions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-generator-function-definitions"
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "basic functionality",
                exec: "function * generator(){\n  yield 5; yield 6;\n};\nvar iterator = generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "generator function expressions",
                exec: "var generator = function * (){\n  yield 5; yield 6;\n};\nvar iterator = generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "correct \"this\" binding",
                exec: "function * generator(){\n  yield this.x; yield this.y;\n};\nvar iterator = { g: generator, x: 5, y: 6 }.g();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "can't use \"this\" with new",
                exec: "function * generator(){\n  yield this.x; yield this.y;\n};\ntry {\n  (new generator()).next();\n}\ncatch (e) {\n  return true;\n}",
            },
            Subtest {
                name: "sending",
                exec: "var sent;\nfunction * generator(){\n  sent = [yield 5, yield 6];\n};\nvar iterator = generator();\niterator.next();\niterator.next(\"foo\");\niterator.next(\"bar\");\nreturn sent[0] === \"foo\" && sent[1] === \"bar\";",
            },
            Subtest {
                name: "%GeneratorPrototype%",
                exec: "function * generatorFn(){}\nvar ownProto = Object.getPrototypeOf(generatorFn());\nvar passed = ownProto === generatorFn.prototype;\n\nvar sharedProto = Object.getPrototypeOf(ownProto);\npassed &= sharedProto !== Object.prototype &&\n  sharedProto === Object.getPrototypeOf(function*(){}.prototype) &&\n  sharedProto.hasOwnProperty('next');\n\nreturn passed;",
            },
            Subtest {
                name: "%GeneratorPrototype% prototype chain",
                exec: "function * generatorFn(){}\nvar g = generatorFn();\nvar ownProto = Object.getPrototypeOf(g);\nvar passed = ownProto === generatorFn.prototype;\n\nvar sharedProto = Object.getPrototypeOf(ownProto);\nvar iterProto = Object.getPrototypeOf(sharedProto);\n\npassed &= iterProto.hasOwnProperty(Symbol.iterator) &&\n  !sharedProto     .hasOwnProperty(Symbol.iterator) &&\n  !ownProto        .hasOwnProperty(Symbol.iterator) &&\n  g[Symbol.iterator]() === g;\n\nreturn passed;",
            },
            Subtest {
                name: "%GeneratorPrototype%.constructor",
                exec: "function * g (){}\nvar iterator = new g.constructor(\"a\",\"b\",\"c\",\"yield a; yield b; yield c;\")(5,6,7);\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 7 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\n\npassed &= g.constructor === (function*(){}).constructor;\nreturn passed;",
            },
            Subtest {
                name: "%GeneratorPrototype%.throw",
                exec: "var passed = false;\nfunction * generator(){\n  try {\n    yield 5; yield 6;\n  } catch(e) {\n    passed = (e === \"foo\");\n  }\n};\nvar iterator = generator();\niterator.next();\niterator.throw(\"foo\");\nreturn passed;",
            },
            Subtest {
                name: "%GeneratorPrototype%.return",
                exec: "function * generator(){\n  yield 5; yield 6;\n};\nvar iterator = generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.return(\"quxquux\");\npassed &= item.value === \"quxquux\" && item.done === true;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield operator precedence",
                exec: "var passed;\nfunction * generator(){\n  passed = yield 0 ? true : false;\n};\nvar iterator = generator();\niterator.next();\niterator.next(true);\nreturn passed;",
            },
            Subtest {
                name: "yield *, arrays",
                exec: "var iterator = (function * generator() {\n  yield * [5, 6];\n}());\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, sparse arrays",
                exec: "var iterator = (function * generator() {\n  yield * [,,];\n}());\nvar item = iterator.next();\nvar passed = item.value === void undefined && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, strings",
                exec: "var iterator = (function * generator() {\n  yield * \"56\";\n}());\nvar item = iterator.next();\nvar passed = item.value === \"5\" && item.done === false;\nitem = iterator.next();\npassed &= item.value === \"6\" && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, astral plane strings",
                exec: "var iterator = (function * generator() {\n  yield * \"𠮷𠮶\";\n}());\nvar item = iterator.next();\nvar passed = item.value === \"𠮷\" && item.done === false;\nitem = iterator.next();\npassed &= item.value === \"𠮶\" && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, generator instances",
                exec: "var iterator = (function * generator() {\n  yield * (function*(){ yield 5; yield 6; yield 7; }());\n}());\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 7 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, generic iterables",
                exec: "var iterator = (function * generator() {\n  yield * global.__createIterableObject([5, 6, 7]);\n}());\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 7 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield *, instances of iterables",
                exec: "var iterator = (function * generator() {\n  yield * Object.create(__createIterableObject([5, 6, 7]));\n}());\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 7 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "yield * on non-iterables is a runtime error",
                exec: "var iterator = (function * generator() {\n  yield * [5];\n}());\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\niterator = (function * generator() {\n  yield * 5;\n}());\ntry {\n  iterator.next();\n} catch (e) {\n  return passed;\n}",
            },
            Subtest {
                name: "yield *, iterator closing",
                exec: "var closed = '';\nvar iter = __createIterableObject([1, 2, 3], {\n  'return': function(){\n    closed += 'a';\n    return {done: true};\n  }\n});\nvar gen = (function* generator(){\n  try {\n    yield *iter;\n  } finally {\n    closed += 'b';\n  }\n})();\ngen.next();\ngen['return']();\nreturn closed === 'ab';",
            },
            Subtest {
                name: "yield *, iterator closing via throw()",
                exec: "var closed = false;\nvar iter = global.__createIterableObject([1, 2, 3], {\n  'throw': undefined,\n  'return': function() {\n    closed = true;\n    return {done: true};\n  }\n});\nvar gen = (function*(){\n  try {\n    yield *iter;\n  } catch(e){}\n})();\ngen.next();\ngen['throw']();\nreturn closed;",
            },
            Subtest {
                name: "shorthand generator methods",
                exec: "var o = {\n  * generator() {\n    yield 5; yield 6;\n  }\n};\nvar iterator = o.generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "string-keyed shorthand generator methods",
                exec: "var o = {\n  * \"foo bar\"() {\n    yield 5; yield 6;\n  }\n};\nvar iterator = o[\"foo bar\"]();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "computed shorthand generators",
                exec: "var garply = \"generator\";\nvar o = {\n  * [garply] () {\n    yield 5; yield 6;\n  }\n};\nvar iterator = o.generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "shorthand generator methods, classes",
                exec: "class C {\n  * generator() {\n    yield 5; yield 6;\n  }\n};\nvar iterator = new C().generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "computed shorthand generators, classes",
                exec: "var garply = \"generator\";\nclass C {\n  * [garply] () {\n    yield 5; yield 6;\n  }\n}\nvar iterator = new C().generator();\nvar item = iterator.next();\nvar passed = item.value === 5 && item.done === false;\nitem = iterator.next();\npassed &= item.value === 6 && item.done === false;\nitem = iterator.next();\npassed &= item.value === void undefined && item.done === true;\nreturn passed;",
            },
            Subtest {
                name: "shorthand generators can't be constructors",
                exec: "class C {\n  * generator() {\n    yield 5; yield 6;\n  }\n};\ntry {\n  Function(\"class D { * constructor() { return {}; } }\");\n} catch(e) {\n  return true;\n}",
            },
        ]
    }
}
