use crate::{
    feature::{Meta, Subtest},
    features::Es6Reflect,
};
impl Meta for Es6Reflect {
    fn name(&self) -> &'static str {
        "Reflect"
    }

    fn key(&self) -> &'static str {
        "es6_reflect"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-ins"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-reflection"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Reflect"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "Reflect.get",
                exec: "return Reflect.get({ qux: 987 }, \"qux\") === 987;",
            },
            Subtest {
                name: "Reflect.set",
                exec: "var obj = {};\nReflect.set(obj, \"quux\", 654);\nreturn obj.quux === 654;",
            },
            Subtest { name: "Reflect.has", exec: "return Reflect.has({ qux: 987 }, \"qux\");" },
            Subtest {
                name: "Reflect.deleteProperty",
                exec: "var obj = { bar: 456 };\nReflect.deleteProperty(obj, \"bar\");\nreturn !(\"bar\" in obj);",
            },
            Subtest {
                name: "Reflect.getOwnPropertyDescriptor",
                exec: "var obj = { baz: 789 };\nvar desc = Reflect.getOwnPropertyDescriptor(obj, \"baz\");\nreturn desc.value === 789 &&\n  desc.configurable && desc.writable && desc.enumerable;",
            },
            Subtest {
                name: "Reflect.defineProperty",
                exec: "var obj = {};\nReflect.defineProperty(obj, \"foo\", { value: 123 });\nreturn obj.foo === 123 &&\n  Reflect.defineProperty(Object.freeze({}), \"foo\", { value: 123 }) === false;",
            },
            Subtest {
                name: "Reflect.getPrototypeOf",
                exec: "return Reflect.getPrototypeOf([]) === Array.prototype;",
            },
            Subtest {
                name: "Reflect.setPrototypeOf",
                exec: "var obj = {};\nReflect.setPrototypeOf(obj, Array.prototype);\nreturn obj instanceof Array;",
            },
            Subtest {
                name: "Reflect.isExtensible",
                exec: "return Reflect.isExtensible({}) &&\n  !Reflect.isExtensible(Object.preventExtensions({}));",
            },
            Subtest {
                name: "Reflect.preventExtensions",
                exec: "var obj = {};\nReflect.preventExtensions(obj);\nreturn !Object.isExtensible(obj);",
            },
            Subtest {
                name: "Reflect.ownKeys, string keys",
                exec: "var obj = Object.create({ C: true });\nobj.A = true;\nObject.defineProperty(obj, 'B', { value: true, enumerable: false });\n\nreturn Reflect.ownKeys(obj).sort() + '' === \"A,B\";",
            },
            Subtest {
                name: "Reflect.ownKeys, symbol keys",
                exec: "var s1 = Symbol(), s2 = Symbol(), s3 = Symbol();\nvar proto = {};\nproto[s1] = true;\nvar obj = Object.create(proto);\nobj[s2] = true;\nObject.defineProperty(obj, s3, { value: true, enumerable: false });\n\nvar keys = Reflect.ownKeys(obj);\nreturn keys.indexOf(s2) >-1 && keys.indexOf(s3) >-1 && keys.length === 2;",
            },
            Subtest {
                name: "Reflect.apply",
                exec: "return Reflect.apply(Array.prototype.push, [1,2], [3,4,5]) === 5;",
            },
            Subtest {
                name: "Reflect.construct",
                exec: "return Reflect.construct(function(a, b, c) {\n  this.qux = a + b + c;\n}, [\"foo\", \"bar\", \"baz\"]).qux === \"foobarbaz\";",
            },
            Subtest {
                name: "Reflect.construct sets new.target meta-property",
                exec: "return Reflect.construct(function(a, b, c) {\n  if (new.target === Object) {\n    this.qux = a + b + c;\n  }\n}, [\"foo\", \"bar\", \"baz\"], Object).qux === \"foobarbaz\";",
            },
            Subtest {
                name: "Reflect.construct creates instances from third argument",
                exec: "function F(){}\nvar obj = Reflect.construct(function(){ this.y = 1; }, [], F);\nreturn obj.y === 1 && obj instanceof F;",
            },
            Subtest {
                name: "Reflect.construct, Array subclassing",
                exec: "function F(){}\nvar obj = Reflect.construct(Array, [], F);\nobj[2] = 'foo';\nreturn obj.length === 3 && obj instanceof F;",
            },
            Subtest {
                name: "Reflect.construct, RegExp subclassing",
                exec: "function F(){}\nvar obj = Reflect.construct(RegExp, [\"baz\",\"g\"], F);\nreturn RegExp.prototype.exec.call(obj, \"foobarbaz\")[0] === \"baz\"\n  && obj.lastIndex === 9 && obj instanceof F;",
            },
            Subtest {
                name: "Reflect.construct, Function subclassing",
                exec: "function F(){}\nvar obj = Reflect.construct(Function, [\"return 2\"], F);\nreturn obj() === 2 && obj instanceof F;",
            },
            Subtest {
                name: "Reflect.construct, Promise subclassing",
                exec: "function F(){}\nvar p1 = Reflect.construct(Promise,[function(resolve, reject) { resolve(\"foo\"); }], F);\nvar p2 = Reflect.construct(Promise,[function(resolve, reject) { reject(\"quux\"); }], F);\nvar score = +(p1 instanceof F && p2 instanceof F);\n\nfunction thenFn(result)  { score += (result === \"foo\");  check(); }\nfunction catchFn(result) { score += (result === \"quux\"); check(); }\nfunction shouldNotRun(result)  { score = -Infinity;   }\n\np1.then = p2.then = Promise.prototype.then;\np1.catch = p2.catch = Promise.prototype.catch;\n\np1.then(thenFn, shouldNotRun);\np2.then(shouldNotRun, catchFn);\np1.catch(shouldNotRun);\np2.catch(catchFn);\n\nfunction check() {\n  if (score === 4) asyncTestPassed();\n}",
            },
        ]
    }
}
