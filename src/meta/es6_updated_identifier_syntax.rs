use crate::feature::{Meta, Subtest};
use crate::features::Es6UpdatedIdentifierSyntax;
impl Meta for Es6UpdatedIdentifierSyntax {
    fn name(&self) -> &'static str {
        "Updated identifier syntax"
    }
    fn key(&self) -> &'static str {
        "es6_updated_identifier_syntax"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-names-and-keywords"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "var ⸯ;", exec :
            "try {\n  eval('var ⸯ');\n} catch(e) {\n  return true;\n}" }, Subtest {
            name : "var 𐋀;", exec : "var 𐋀;\nreturn true;" }, Subtest { name :
            "no escaped reserved words as identifiers", exec :
            "var \\u0061;\ntry {\n  eval('var v\\\\u0061r');\n} catch(e) {\n  return true;\n}"
            },
        ]
    }
}
