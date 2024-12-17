use crate::feature::{Meta, Subtest};
use crate::features::Es2016NestedRestDestructuringDeclarations;
impl Meta for Es2016NestedRestDestructuringDeclarations {
    fn name(&self) -> &'static str {
        "nested rest destructuring, declarations"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#Nested_object_and_array_destructuring"
    }
    fn exec(&self) -> &'static str {
        "var [x, ...[y, ...z]] = [1,2,3,4];\nreturn x === 1 && y === 2 && z + '' === '3,4';"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
