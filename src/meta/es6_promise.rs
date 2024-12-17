use crate::feature::{Meta, Subtest};
use crate::features::Es6Promise;
impl Meta for Es6Promise {
    fn name(&self) -> &'static str {
        "Promise"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-promise-objects"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var p1 = new Promise(function(resolve, reject) { resolve(\"foo\"); });\nvar p2 = new Promise(function(resolve, reject) { reject(\"quux\"); });\nvar score = 0;\n\nfunction thenFn(result)  { score += (result === \"foo\");  check(); }\nfunction catchFn(result) { score += (result === \"quux\"); check(); }\nfunction shouldNotRun(result)  { score = -Infinity;   }\n\np1.then(thenFn, shouldNotRun);\np2.then(shouldNotRun, catchFn);\np1.catch(shouldNotRun);\np2.catch(catchFn);\n\np1.then(function() {\n  // Promise.prototype.then() should return a new Promise\n  score += p1.then() !== p1;\n  check();\n});\n\nfunction check() {\n  if (score === 4) asyncTestPassed();\n}",
            }, Subtest { name : "constructor requires new", exec :
            "new Promise(function(){});\ntry {\n  Promise(function(){});\n  return false;\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "Promise.prototype isn't an instance", exec :
            "new Promise(function(){});\ntry {\n  Promise.prototype.then(function(){});\n} catch (e) {\n  return true;\n}",
            }, Subtest { name : "Promise.all", exec :
            "var fulfills = Promise.all([\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"foo\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"bar\"); })\n]);\nvar rejects = Promise.all([\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"baz\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"qux\"); })\n]);\nvar score = 0;\nfulfills.then(function(result) { score += (result + \"\" === \"foo,bar\"); check(); });\nrejects.catch(function(result) { score += (result === \"qux\"); check(); });\n\nfunction check() {\n  if (score === 2) asyncTestPassed();\n}",
            }, Subtest { name : "Promise.all, generic iterables", exec :
            "var fulfills = Promise.all(global.__createIterableObject([\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"foo\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"bar\"); })\n]));\nvar rejects = Promise.all(global.__createIterableObject([\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"baz\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"qux\"); })\n]));\nvar score = 0;\nfulfills.then(function(result) { score += (result + \"\" === \"foo,bar\"); check(); });\nrejects.catch(function(result) { score += (result === \"qux\"); check(); });\n\nfunction check() {\n  if (score === 2) asyncTestPassed();\n}",
            }, Subtest { name : "Promise.race", exec :
            "var fulfills = Promise.race([\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"foo\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"bar\"); })\n]);\nvar rejects = Promise.race([\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"baz\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"qux\"); })\n]);\nvar score = 0;\nfulfills.then(function(result) { score += (result === \"foo\"); check(); });\nrejects.catch(function(result) { score += (result === \"baz\"); check(); });\n\nfunction check() {\n  if (score === 2) asyncTestPassed();\n}",
            }, Subtest { name : "Promise.race, generic iterables", exec :
            "var fulfills = Promise.race(global.__createIterableObject([\n  new Promise(function(resolve)   { setTimeout(resolve,1000,\"foo\"); }),\n  new Promise(function(_, reject) { setTimeout(reject, 2000,\"bar\"); })\n]));\nvar rejects = Promise.race(global.__createIterableObject([\n  new Promise(function(_, reject) { setTimeout(reject, 1000,\"baz\"); }),\n  new Promise(function(resolve)   { setTimeout(resolve,2000,\"qux\"); })\n]));\nvar score = 0;\nfulfills.then(function(result) { score += (result === \"foo\"); check(); });\nrejects.catch(function(result) { score += (result === \"baz\"); check(); });\n\nfunction check() {\n  if (score === 2) asyncTestPassed();\n}",
            }, Subtest { name : "Promise[Symbol.species]", exec :
            "var prop = Object.getOwnPropertyDescriptor(Promise, Symbol.species);\nreturn 'get' in prop && Promise[Symbol.species] === Promise;",
            },
        ]
    }
}
