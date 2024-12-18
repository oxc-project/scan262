use crate::{
    feature::{Meta, Subtest},
    features::Es2016NestedRestDestructuringParameters,
};
impl Meta for Es2016NestedRestDestructuringParameters {
    fn name(&self) -> &'static str {
        "nested rest destructuring, parameters"
    }

    fn key(&self) -> &'static str {
        "es2016_nested_rest_destructuring_parameters"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2016 misc"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-destructuring-assignment"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "return function ([x, ...[y, ...z]]) {\n  return x === 1 && y === 2 && z + '' === '3,4';\n}([1,2,3,4]);"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
