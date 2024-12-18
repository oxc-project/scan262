use crate::{
    feature::{Meta, Subtest},
    features::Es2016GeneratorThrowCaughtByInnerGenerator,
};
impl Meta for Es2016GeneratorThrowCaughtByInnerGenerator {
    fn name(&self) -> &'static str {
        "generator throw() caught by inner generator"
    }

    fn key(&self) -> &'static str {
        "es2016_generator_throw_caught_by_inner_generator"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2016 misc"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-generatorfunction-objects"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*#IteratorResult_object_returned_instead_of_throwing"
    }

    fn exec(&self) -> &'static str {
        "function * generator() {\n  yield * (function * () {\n    try {\n      yield 'foo';\n    }\n    catch (e) {\n      return;\n    }\n  }());\n  yield 'bar';\n}\nvar iter = generator();\niter.next();\nreturn iter['throw']().value === 'bar';"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
