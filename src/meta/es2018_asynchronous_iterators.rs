use crate::feature::{Meta, Subtest};
use crate::features::Es2018AsynchronousIterators;
impl Meta for Es2018AsynchronousIterators {
    fn name(&self) -> &'static str {
        "Asynchronous Iterators"
    }
    fn key(&self) -> &'static str {
        "es2018_asynchronous_iterators"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-async-iteration"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "async generators", exec :
            "async function*generator() {\n  yield 42;\n}\n\nvar iterator = generator();\niterator.next().then(function (step) {\n  if(iterator[Symbol.asyncIterator]() === iterator && step.done === false && step.value === 42)asyncTestPassed();\n});",
            }, Subtest { name : "for-await-of loops", exec :
            "var asyncIterable = {};\nasyncIterable[Symbol.asyncIterator] = function () {\n  var i = 0;\n  return {\n    next: function () {\n      switch(++i) {\n        case 1: return Promise.resolve({done: false, value: 'a'});\n        case 2: return Promise.resolve({done: false, value: 'b'});\n      } return Promise.resolve({done: true});\n    }\n  };\n};\n\n(async function () {\n  var result = '';\n  for await(var value of asyncIterable)result += value;\n  if(result === 'ab')asyncTestPassed();\n})();",
            },
        ]
    }
}
