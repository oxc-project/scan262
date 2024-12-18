use crate::feature::{Meta, Subtest};
use crate::features::Es2016GeneratorFunctionsCanTBeUsedWithNew;
impl Meta for Es2016GeneratorFunctionsCanTBeUsedWithNew {
    fn name(&self) -> &'static str {
        "generator functions can't be used with \"new\""
    }
    fn key(&self) -> &'static str {
        "es2016_generator_functions_can_t_be_used_with_new"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-createdynamicfunction"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*#Generators_are_not_constructable"
    }
    fn exec(&self) -> &'static str {
        "function * generator() {\n  yield 3;\n}\ntry {\n  new generator();\n} catch (e) {\n  return true;\n}"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
