use crate::feature::{Meta, Subtest};
use crate::features::Es6PromiseIsSubclassable;
impl Meta for Es6PromiseIsSubclassable {
    fn name(&self) -> &'static str {
        "Promise is subclassable"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "subclassing"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-function-constructor"
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
            Subtest { name : "basic functionality", exec :
            "class P extends Promise {}\nvar p1 = new P(function(resolve, reject) { resolve(\"foo\"); });\nvar p2 = new P(function(resolve, reject) { reject(\"quux\"); });\nvar score = +(p1 instanceof P);\n\nfunction thenFn(result)  { score += (result === \"foo\");  check(); }\nfunction catchFn(result) { score += (result === \"quux\"); check(); }\nfunction shouldNotRun(result)  { score = -Infinity;   }\n\np1.then(thenFn, shouldNotRun);\np2.then(shouldNotRun, catchFn);\np1.catch(shouldNotRun);\np2.catch(catchFn);\n\np1.then(function() {\n  // P.prototype.then() should return a new P\n  score += p1.then() instanceof P && p1.then() !== p1;\n  check();\n});\n\nfunction check() {\n  if (score === 5) asyncTestPassed();\n}",
            }, Subtest { name : "correct prototype chain", exec :
            "class C extends Promise {}\nvar c = new C(function(resolve, reject) { resolve(\"foo\"); });\nreturn c instanceof C && c instanceof Promise && Object.getPrototypeOf(C) === Promise;",
            }, Subtest { name : "Promise.all", exec :
            "class P extends Promise {}\nvar fulfills = P.all([\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"foo\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"bar\"); })\n]);\nvar rejects = P.all([\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"baz\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"qux\"); })\n]);\nvar score = +(fulfills instanceof P);\nfulfills.then(function(result) { score += (result + \"\" === \"foo,bar\"); check(); });\nrejects.catch(function(result) { score += (result === \"qux\"); check(); });\n\nfunction check() {\n  if (score === 3) asyncTestPassed();\n}",
            }, Subtest { name : "Promise.race", exec :
            "class P extends Promise {}\nvar fulfills = P.race([\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"foo\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"bar\"); })\n]);\nvar rejects = P.race([\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"baz\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"qux\"); })\n]);\nvar score = +(fulfills instanceof P);\nfulfills.then(function(result) { score += (result === \"foo\"); check(); });\nrejects.catch(function(result) { score += (result === \"baz\"); check(); });\n\nfunction check() {\n  if (score === 3) asyncTestPassed();\n}",
            },
        ]
    }
}
