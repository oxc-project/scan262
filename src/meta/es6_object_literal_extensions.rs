use crate::{
    feature::{Meta, Subtest},
    features::Es6ObjectLiteralExtensions,
};
impl Meta for Es6ObjectLiteralExtensions {
    fn name(&self) -> &'static str {
        "object literal extensions"
    }

    fn key(&self) -> &'static str {
        "es6_object_literal_extensions"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "syntax"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-object-initialiser"
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer#New_notations_in_ECMAScript_2015"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "computed properties",
                exec: "var x = 'y';\nreturn ({ [x]: 1 }).y === 1;",
            },
            Subtest {
                name: "shorthand properties",
                exec: "var a = 7, b = 8, c = {a,b};\nreturn c.a === 7 && c.b === 8;",
            },
            Subtest {
                name: "shorthand methods",
                exec: "return ({ y() { return 2; } }).y() === 2;",
            },
            Subtest {
                name: "string-keyed shorthand methods",
                exec: "return ({ \"foo bar\"() { return 4; } })[\"foo bar\"]() === 4;",
            },
            Subtest {
                name: "computed shorthand methods",
                exec: "var x = 'y';\nreturn ({ [x](){ return 1 } }).y() === 1;",
            },
            Subtest {
                name: "computed accessors",
                exec: "var x = 'y',\n    valueSet,\n    obj = {\n      get [x] () { return 1 },\n      set [x] (value) { valueSet = value }\n    };\nobj.y = 'foo';\nreturn obj.y === 1 && valueSet === 'foo';",
            },
        ]
    }
}
