use crate::feature::{Meta, Subtest};
use crate::features::Es2018PromisePrototypeFinally;
impl Meta for Es2018PromisePrototypeFinally {
    fn name(&self) -> &'static str {
        "Promise.prototype.finally"
    }
    fn key(&self) -> &'static str {
        "es2018_promise_prototype_finally"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-promise-finally"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/finally"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic support", exec :
            "var p1 = Promise.resolve(\"foo\");\nvar p2 = Promise.reject(\"bar\");\nvar score = 0;\nfunction thenFn(result) {\n  score += (result === \"foo\");\n  check();\n}\nfunction catchFn(result) {\n  score += (result === \"bar\");\n  check();\n}\nfunction finallyFn() {\n  score += (arguments.length === 0);\n  check();\n}\np1.then(thenFn);\np1.finally(finallyFn);\np1.finally(function () {\n  // should return a new Promise\n  score += p1.finally() !== p1;\n  check();\n});\np2.catch (catchFn);\np2.finally(finallyFn);\nfunction check() {\n  if (score === 5) asyncTestPassed();\n}",
            }, Subtest { name : "don't change resolution value", exec :
            "var score = 0;\nfunction thenFn(result)  {\n  score += (result === \"foo\");\n  check();\n}\nfunction catchFn(result) {\n  score += (result === \"bar\");\n  check();\n}\nfunction finallyFn() {\n  score++;\n  check();\n  return Promise.resolve(\"foobar\");\n}\nPromise.resolve(\"foo\").finally(finallyFn).then(thenFn);\nPromise.reject(\"bar\").finally(finallyFn).catch(catchFn);\nfunction check() {\n  if (score === 4) asyncTestPassed();\n}",
            }, Subtest { name : "change rejection value", exec :
            "var score = 0;\nPromise\n  .reject(\"foobar\")\n  .finally(function () {\n    return Promise.reject(\"foo\");\n  })\n  .catch (function (result) {\n    score += (result === \"foo\");\n    check();\n    return Promise.reject(\"foobar\");\n  })\n  .finally(function () {\n    throw new Error('bar');\n  })\n  .catch (function (result) {\n    score += (result.message === \"bar\");\n    check();\n  });\nfunction check() {\n  if (score === 2) asyncTestPassed();\n}",
            },
        ]
    }
}
