use crate::{
    feature::{Meta, Subtest},
    features::Es6BlockLevelFunctionDeclaration,
};
impl Meta for Es6BlockLevelFunctionDeclaration {
    fn name(&self) -> &'static str {
        "block-level function declaration"
    }

    fn key(&self) -> &'static str {
        "es6_block_level_function_declaration"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "bindings"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-functiondeclarationinstantiation"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "'use strict';\nif (f() !== 1) return false;\nfunction f() { return 1; }\n{\n  if (f() !== 2) return false;\n  function f() { return 2; }\n  if (f() !== 2) return false;\n}\nif (f() !== 1) return false;\nreturn true;"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
