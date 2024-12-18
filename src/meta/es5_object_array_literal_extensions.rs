use crate::{
    feature::{Meta, Subtest},
    features::Es5ObjectArrayLiteralExtensions,
};
impl Meta for Es5ObjectArrayLiteralExtensions {
    fn name(&self) -> &'static str {
        "Object/array literal extensions"
    }

    fn key(&self) -> &'static str {
        "es5_object_array_literal_extensions"
    }

    fn target(&self) -> &'static str {
        "es5"
    }

    fn category(&self) -> &'static str {
        ""
    }

    fn spec(&self) -> &'static str {
        ""
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name: "Getter accessors", exec: "return ({ get x(){ return 1 } }).x === 1;" },
            Subtest {
                name: "Setter accessors",
                exec: "var value = 0;\n({ set x(v){ value = v; } }).x = 1;\nreturn value === 1;",
            },
            Subtest {
                name: "Trailing commas in object literals",
                exec: "return { a: true, }.a === true;",
            },
            Subtest {
                name: "Trailing commas in array literals",
                exec: "return [1,].length === 1;",
            },
            Subtest {
                name: "Reserved words as property names",
                exec: "return ({ if: 1 }).if === 1;",
            },
        ]
    }
}
