use crate::{
    feature::{Meta, Subtest},
    features::Es2017AsyncFunctions,
};
impl Meta for Es2017AsyncFunctions {
    fn name(&self) -> &'static str {
        "async functions"
    }

    fn key(&self) -> &'static str {
        "es2017_async_functions"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2017 features"
    }

    fn spec(&self) -> &'static str {
        "https://tc39.github.io/ecma262/#sec-async-function-definitions"
    }

    fn significance(&self) -> &'static str {
        "large"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "return",
                exec: "async function a() {\n  return \"foo\";\n}\nvar p = a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.then(function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
            Subtest {
                name: "throw",
                exec: "async function a() {\n  throw \"foo\";\n}\nvar p = a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.catch (function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
            Subtest {
                name: "no line break between async and function",
                exec: "async function a() {}\ntry { function (\"async\\n function a() {await 0}\")(); } catch (e) { return true; }",
            },
            Subtest {
                name: "no \"prototype\" property",
                exec: "async function a() {};\nreturn !a.hasOwnProperty(\"prototype\");",
            },
            Subtest {
                name: "await",
                exec: "(async function () {\n  await Promise.resolve();\n  var a1 = await new Promise(function (resolve) { setTimeout(resolve,800,\"foo\"); });\n  var a2 = await new Promise(function (resolve) { setTimeout(resolve,800,\"bar\"); });\n  if (a1 + a2 === \"foobar\") {\n    asyncTestPassed();\n  }\n}());",
            },
            Subtest {
                name: "await, rejection",
                exec: "(async function () {\n  await Promise.resolve();\n  try {\n    var a1 = await new Promise(function (_, reject) { setTimeout(reject,800,\"foo\"); });\n  } catch (e) {\n    if (e === \"foo\") {\n      asyncTestPassed();\n    }\n  }\n}());",
            },
            Subtest {
                name: "must await a value",
                exec: "async function a() { await Promise.resolve(); }\ntry { function (\"(async function a() { await; }())\")(); } catch (e) { return true; }",
            },
            Subtest {
                name: "can await non-Promise values",
                exec: "(async function () {\n  await Promise.resolve();\n  var e = await \"foo\";\n  if (e === \"foo\") {\n    asyncTestPassed();\n  }\n}());",
            },
            Subtest {
                name: "cannot await in parameters",
                exec: "async function a() { await Promise.resolve(); }\ntry { function (\"(async function a(b = await Promise.resolve()) {}())\")(); } catch (e) { return true; }",
            },
            Subtest {
                name: "async methods, object literals",
                exec: "var o = {\n  async a() { return await Promise.resolve(\"foo\"); }\n};\nvar p = o.a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.then(function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
            Subtest {
                name: "async methods, classes",
                exec: "class C {\n  async a() { return await Promise.resolve(\"foo\"); }\n};\nvar p = new C().a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.then(function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
            Subtest {
                name: "async arrow functions in methods, classes",
                exec: "function doSomething(callback) {\n  callback();\n}\nclass C {\n  a() {\n    doSomething(async () => {\n      await 1;\n      asyncTestPassed();\n    });\n  }\n};\nvar p = new C().a();",
            },
            Subtest {
                name: "async arrow functions",
                exec: "var a = async () => await Promise.resolve(\"foo\");\nvar p = a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.then(function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
            Subtest {
                name: "correct prototype chain",
                exec: "var asyncFunctionProto = Object.getPrototypeOf(async function () {});\nreturn asyncFunctionProto !== function () {}.prototype\n  && Object.getPrototypeOf(asyncFunctionProto) === Function.prototype;",
            },
            Subtest {
                name: "async function prototype, Symbol.toStringTag",
                exec: "return Object.getPrototypeOf(async function () {})[Symbol.toStringTag] === \"AsyncFunction\";",
            },
            Subtest {
                name: "async function constructor",
                exec: "var a = async function () {}.constructor(\"return 'foo';\");\nvar p = a();\nif (!(p instanceof Promise)) {\n  return false;\n}\np.then(function (result) {\n  if (result === \"foo\") {\n    asyncTestPassed();\n  }\n});",
            },
        ]
    }
}
