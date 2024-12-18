use crate::{
    feature::{Meta, Subtest},
    features::Es6StringPrototypeMethods,
};
impl Meta for Es6StringPrototypeMethods {
    fn name(&self) -> &'static str {
        "String.prototype methods"
    }

    fn key(&self) -> &'static str {
        "es6_string_prototype_methods"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-in extensions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-properties-of-the-string-prototype-object"
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
            Subtest {
                name: "String.prototype.codePointAt",
                exec: "return typeof String.prototype.codePointAt === 'function';",
            },
            Subtest {
                name: "String.prototype.normalize",
                exec: "return typeof String.prototype.normalize === \"function\"\n  && \"c\\u0327\\u0301\".normalize(\"NFC\") === \"\\u1e09\"\n  && \"\\u1e09\".normalize(\"NFD\") === \"c\\u0327\\u0301\";",
            },
            Subtest {
                name: "String.prototype.repeat",
                exec: "return typeof String.prototype.repeat === 'function'\n  && \"foo\".repeat(3) === \"foofoofoo\";",
            },
            Subtest {
                name: "String.prototype.startsWith",
                exec: "return typeof String.prototype.startsWith === 'function'\n  && \"foobar\".startsWith(\"foo\");",
            },
            Subtest {
                name: "String.prototype.startsWith throws on RegExp",
                exec: "try {\n  \"a\".startsWith(/./);\n} catch(e) {\n  return typeof String.prototype.startsWith === 'function';\n}",
            },
            Subtest {
                name: "String.prototype.endsWith",
                exec: "return typeof String.prototype.endsWith === 'function'\n  && \"foobar\".endsWith(\"bar\");",
            },
            Subtest {
                name: "String.prototype.endsWith throws on RegExp",
                exec: "try {\n  \"a\".endsWith(/./);\n} catch(e) {\n  return typeof String.prototype.endsWith === 'function';\n}",
            },
            Subtest {
                name: "String.prototype.includes",
                exec: "return typeof String.prototype.includes === 'function'\n  && \"foobar\".includes(\"oba\");",
            },
            Subtest {
                name: "String.prototype[Symbol.iterator]",
                exec: "return typeof String.prototype[Symbol.iterator] === 'function';",
            },
            Subtest {
                name: "String iterator prototype chain",
                exec: "// Iterator instance\nvar iterator = ''[Symbol.iterator]();\n// %StringIteratorPrototype%\nvar proto1 = Object.getPrototypeOf(iterator);\n// %IteratorPrototype%\nvar proto2 = Object.getPrototypeOf(proto1);\n\nreturn proto2.hasOwnProperty(Symbol.iterator) &&\n  !proto1    .hasOwnProperty(Symbol.iterator) &&\n  !iterator  .hasOwnProperty(Symbol.iterator) &&\n  iterator[Symbol.iterator]() === iterator;",
            },
        ]
    }
}
