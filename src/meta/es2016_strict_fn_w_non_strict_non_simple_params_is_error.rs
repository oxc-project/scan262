use crate::feature::{Meta, Subtest};
use crate::features::Es2016StrictFnWNonStrictNonSimpleParamsIsError;
impl Meta for Es2016StrictFnWNonStrictNonSimpleParamsIsError {
    fn name(&self) -> &'static str {
        "strict fn w/ non-strict non-simple params is error"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-functiondeclarationinstantiation"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "function foo(...a) {}\ntry {\n  function (\"function bar(...a) {'use strict';}\")();\n} catch (e) {\nreturn true;\n}"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}