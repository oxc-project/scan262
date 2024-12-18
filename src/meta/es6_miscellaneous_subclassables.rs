use crate::{
    feature::{Meta, Subtest},
    features::Es6MiscellaneousSubclassables,
};
impl Meta for Es6MiscellaneousSubclassables {
    fn name(&self) -> &'static str {
        "miscellaneous subclassables"
    }

    fn key(&self) -> &'static str {
        "es6_miscellaneous_subclassables"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "subclassing"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-boolean-constructor"
    }

    fn significance(&self) -> &'static str {
        "tiny"
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
                name: "Boolean is subclassable",
                exec: "class C extends Boolean {}\nvar c = new C(true);\nreturn c instanceof Boolean\n  && c instanceof C\n  && c == true; // eslint-disable-line eqeqeq",
            },
            Subtest {
                name: "Number is subclassable",
                exec: "class C extends Number {}\nvar c = new C(6);\nreturn c instanceof Number\n  && c instanceof C\n  && +c === 6;",
            },
            Subtest {
                name: "String is subclassable",
                exec: "class C extends String {}\nvar c = new C(\"golly\");\nreturn c instanceof String\n  && c instanceof C\n  && c + '' === \"golly\"\n  && c[0] === \"g\"\n  && c.length === 5;",
            },
            Subtest {
                name: "Error is subclassable",
                exec: "class C extends Error {}\nvar c = new C();\nreturn c instanceof Error\n  && c instanceof C\n  && Object.prototype.toString.call(c) === \"[object Error]\";",
            },
            Subtest {
                name: "Map is subclassable",
                exec: "var key = {};\nclass M extends Map {}\nvar map = new M();\n\nmap.set(key, 123);\n\nreturn map instanceof M && map.has(key) && map.get(key) === 123;",
            },
            Subtest {
                name: "Set is subclassable",
                exec: "var obj = {};\nclass S extends Set {}\nvar set = new S();\n\nset.add(123);\nset.add(123);\n\nreturn set instanceof S && set.has(123);",
            },
        ]
    }
}
