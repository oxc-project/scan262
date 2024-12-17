use crate::feature::{Meta, Subtest};
use crate::features::Es6Proxy;
impl Meta for Es6Proxy {
    fn name(&self) -> &'static str {
        "Proxy"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-ins"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-proxy-object-internal-methods-and-internal-slots"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "constructor requires new", exec :
            "new Proxy({}, {});\ntry {\n  Proxy({}, {});\n  return false;\n} catch(e) {\n  return true;\n}",
            }, Subtest { name : "no \"prototype\" property", exec :
            "new Proxy({}, {});\nreturn !Proxy.hasOwnProperty('prototype');", }, Subtest
            { name : "\"get\" handler", exec :
            "var proxied = { };\nvar proxy = new Proxy(proxied, {\n  get: function (t, k, r) {\n    return t === proxied && k === \"foo\" && r === proxy && 5;\n  }\n});\nreturn proxy.foo === 5;",
            }, Subtest { name : "\"get\" handler, instances of proxies", exec :
            "var proxied = { };\nvar proxy = Object.create(new Proxy(proxied, {\n  get: function (t, k, r) {\n    return t === proxied && k === \"foo\" && r === proxy && 5;\n  }\n}));\nreturn proxy.foo === 5;",
            }, Subtest { name : "\"get\" handler invariants", exec :
            "var passed = false;\nvar proxied = { };\nvar proxy = new Proxy(proxied, {\n  get: function () {\n    passed = true;\n    return 4;\n  }\n});\n// The value reported for a property must be the same as the value of the corresponding\n// target object property if the target object property is a non-writable,\n// non-configurable own data property.\nObject.defineProperty(proxied, \"foo\", { value: 5, enumerable: true });\ntry {\n  proxy.foo;\n  return false;\n}\ncatch(e) {}\n// The value reported for a property must be undefined if the corresponding target\n// object property is a non-configurable own accessor property that has undefined\n// as its [[Get]] attribute.\nObject.defineProperty(proxied, \"bar\",\n  { set: function(){}, enumerable: true });\ntry {\n  proxy.bar;\n  return false;\n}\ncatch(e) {}\nreturn passed;",
            }, Subtest { name : "\"set\" handler", exec :
            "var proxied = { };\nvar passed = false;\nvar proxy = new Proxy(proxied, {\n  set: function (t, k, v, r) {\n    passed = t === proxied && k + v === \"foobar\" && r === proxy;\n  }\n});\nproxy.foo = \"bar\";\nreturn passed;",
            }, Subtest { name : "\"set\" handler, instances of proxies", exec :
            "var proxied = { };\nvar passed = false;\nvar proxy = Object.create(new Proxy(proxied, {\n  set: function (t, k, v, r) {\n    passed = t === proxied && k + v === \"foobar\" && r === proxy;\n  }\n}));\nproxy.foo = \"bar\";\nreturn passed;",
            }, Subtest { name : "\"set\" handler invariants", exec :
            "var passed = false;\nnew Proxy({},{});\n// Cannot change the value of a property to be different from the value of\n// the corresponding target object if the corresponding target object\n// property is a non-writable, non-configurable own data property.\nvar proxied = {};\nvar proxy = new Proxy(proxied, {\n  set: function () {\n    passed = true;\n    return true;\n  }\n});\nObject.defineProperty(proxied, \"foo\", { value: 2, enumerable: true });\nproxy.foo = 2;\ntry {\n  proxy.foo = 4;\n  return false;\n} catch(e) {}\n// Cannot set the value of a property if the corresponding target\n// object property is a non-configurable own accessor property\n// that has undefined as its [[Set]] attribute.\nObject.defineProperty(proxied, \"bar\",\n  { get: function(){}, enumerable: true });\ntry {\n  proxy.bar = 2;\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"has\" handler", exec :
            "var proxied = {};\nvar passed = false;\n\"foo\" in new Proxy(proxied, {\n  has: function (t, k) {\n    passed = t === proxied && k === \"foo\";\n  }\n});\nreturn passed;",
            }, Subtest { name : "\"has\" handler, instances of proxies", exec :
            "var proxied = {};\nvar passed = false;\n\"foo\" in Object.create(new Proxy(proxied, {\n  has: function (t, k) {\n    passed = t === proxied && k === \"foo\";\n  }\n}));\nreturn passed;",
            }, Subtest { name : "\"has\" handler invariants", exec :
            "var passed = false;\nnew Proxy({},{});\n// A property cannot be reported as non-existent, if it exists as a\n// non-configurable own property of the target object.\nvar proxied = {};\nvar proxy = new Proxy(proxied, {\n  has: function () {\n    passed = true;\n    return false;\n  }\n});\nObject.defineProperty(proxied, \"foo\", { value: 2, writable: true, enumerable: true });\ntry {\n  'foo' in proxy;\n  return false;\n} catch(e) {}\n// A property cannot be reported as non-existent, if it exists as an\n// own property of the target object and the target object is not extensible.\nproxied.bar = 2;\nObject.preventExtensions(proxied);\ntry {\n  'bar' in proxy;\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"deleteProperty\" handler", exec :
            "var proxied = {};\nvar passed = false;\ndelete new Proxy(proxied, {\n  deleteProperty: function (t, k) {\n    passed = t === proxied && k === \"foo\";\n  }\n}).foo;\nreturn passed;",
            }, Subtest { name : "\"deleteProperty\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\n// A property cannot be reported as deleted, if it exists as a non-configurable\n// own property of the target object.\nvar proxied = {};\nObject.defineProperty(proxied, \"foo\", { value: 2, writable: true, enumerable: true });\ntry {\n  delete new Proxy(proxied, {\n    deleteProperty: function () {\n      passed = true;\n      return true;\n    }\n  }).foo;\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"getOwnPropertyDescriptor\" handler", exec :
            "var proxied = {};\nvar fakeDesc = { value: \"foo\", configurable: true };\nvar returnedDesc = Object.getOwnPropertyDescriptor(\n  new Proxy(proxied, {\n    getOwnPropertyDescriptor: function (t, k) {\n      return t === proxied && k === \"foo\" && fakeDesc;\n    }\n  }),\n  \"foo\"\n);\nreturn (returnedDesc.value     === fakeDesc.value\n  && returnedDesc.configurable === fakeDesc.configurable\n  && returnedDesc.writable     === false\n  && returnedDesc.enumerable   === false);",
            }, Subtest { name : "\"getOwnPropertyDescriptor\" handler invariants", exec :
            "var passed = false;\nnew Proxy({},{});\n// A property cannot be reported as non-existent, if it exists as a non-configurable\n// own property of the target object.\nvar proxied = {};\nvar proxy = new Proxy(proxied, {\n  getOwnPropertyDescriptor: function () {\n    passed = true;\n    return undefined;\n  }\n});\nObject.defineProperty(proxied, \"foo\", { value: 2, writable: true, enumerable: true });\ntry {\n  Object.getOwnPropertyDescriptor(proxy, \"foo\");\n  return false;\n} catch(e) {}\n// A property cannot be reported as non-existent, if it exists as an own property\n// of the target object and the target object is not extensible.\nproxied.bar = 3;\nObject.preventExtensions(proxied);\ntry {\n  Object.getOwnPropertyDescriptor(proxy, \"bar\");\n  return false;\n} catch(e) {}\n// A property cannot be reported as existent, if it does not exists as an own property\n// of the target object and the target object is not extensible.\ntry {\n  Object.getOwnPropertyDescriptor(new Proxy(proxied, {\n    getOwnPropertyDescriptor: function() {\n      return { value: 2, configurable: true, writable: true, enumerable: true };\n    }}), \"baz\");\n  return false;\n} catch(e) {}\n// A property cannot be reported as non-configurable, if it does not exists as an own\n// property of the target object or if it exists as a configurable own property of\n// the target object.\ntry {\n  Object.getOwnPropertyDescriptor(new Proxy({}, {\n    getOwnPropertyDescriptor: function() {\n      return { value: 2, configurable: false, writable: true, enumerable: true };\n    }}), \"baz\");\n  return false;\n} catch(e) {}\ntry {\n  Object.getOwnPropertyDescriptor(new Proxy({baz:1}, {\n    getOwnPropertyDescriptor: function() {\n      return { value: 1, configurable: false, writable: true, enumerable: true };\n    }}), \"baz\");\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"defineProperty\" handler", exec :
            "var proxied = {};\nvar passed = false;\nObject.defineProperty(\n  new Proxy(proxied, {\n    defineProperty: function (t, k, d) {\n      passed = t === proxied && k === \"foo\" && d.value === 5;\n      return true;\n    }\n  }),\n  \"foo\",\n  { value: 5, configurable: true }\n);\nreturn passed;",
            }, Subtest { name : "\"defineProperty\" handler invariants", exec :
            "var passed = false;\nnew Proxy({},{});\n// A property cannot be added, if the target object is not extensible.\nvar proxied = Object.preventExtensions({});\nvar proxy = new Proxy(proxied, {\n  defineProperty: function() {\n    passed = true;\n    return true;\n  }\n});\ntry {\n  Object.defineProperty(proxy, \"foo\", { value: 2 });\n  return false;\n} catch(e) {}\n// A property cannot be non-configurable, unless there exists a corresponding\n// non-configurable own property of the target object.\ntry {\n  Object.defineProperty(\n    new Proxy({ bar: true }, {\n      defineProperty: function () {\n        return true;\n      }\n    }),\n    \"bar\",\n    { value: 5, configurable: false, writable: true, enumerable: true }\n  );\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"getPrototypeOf\" handler", exec :
            "var proxied = {};\nvar fakeProto = {};\nvar proxy = new Proxy(proxied, {\n  getPrototypeOf: function (t) {\n    return t === proxied && fakeProto;\n  }\n});\nreturn Object.getPrototypeOf(proxy) === fakeProto;",
            }, Subtest { name : "\"getPrototypeOf\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\n// If the target object is not extensible, [[GetPrototypeOf]] applied to the proxy object\n// must return the same value as [[GetPrototypeOf]] applied to the proxy object's target object.\ntry {\n  Object.getPrototypeOf(new Proxy(Object.preventExtensions({}), {\n    getPrototypeOf: function () {\n      passed = true;\n      return {};\n    }\n  }));\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"setPrototypeOf\" handler", exec :
            "var proxied = {};\nvar newProto = {};\nvar passed = false;\nObject.setPrototypeOf(\n  new Proxy(proxied, {\n    setPrototypeOf: function (t, p) {\n      passed = t === proxied && p === newProto;\n      return true;\n    }\n  }),\n  newProto\n);\nreturn passed;",
            }, Subtest { name : "\"setPrototypeOf\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\nObject.setPrototypeOf({},{});\n// If the target object is not extensible, the argument value must be the\n// same as the result of [[GetPrototypeOf]] applied to target object.\ntry {\n  Object.setPrototypeOf(\n    new Proxy(Object.preventExtensions({}), {\n      setPrototypeOf: function () {\n        passed = true;\n        return true;\n      }\n    }),{});\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"isExtensible\" handler", exec :
            "var proxied = {};\nvar passed = false;\nObject.isExtensible(\n  new Proxy(proxied, {\n    isExtensible: function (t) {\n      passed = t === proxied; return true;\n    }\n  })\n);\nreturn passed;",
            }, Subtest { name : "\"isExtensible\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\n// [[IsExtensible]] applied to the proxy object must return the same value\n// as [[IsExtensible]] applied to the proxy object's target object with the same argument.\ntry {\n  Object.isExtensible(new Proxy({}, {\n    isExtensible: function (t) {\n      passed = true;\n      return false;\n    }\n  }));\n  return false;\n} catch(e) {}\ntry {\n  Object.isExtensible(new Proxy(Object.preventExtensions({}), {\n    isExtensible: function (t) {\n      return true;\n    }\n  }));\n  return false;\n} catch(e) {}\nreturn true;",
            }, Subtest { name : "\"preventExtensions\" handler", exec :
            "var proxied = {};\nvar passed = false;\nObject.preventExtensions(\n  new Proxy(proxied, {\n    preventExtensions: function (t) {\n      passed = t === proxied;\n      return Object.preventExtensions(proxied);\n    }\n  })\n);\nreturn passed;",
            }, Subtest { name : "\"preventExtensions\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\n// [[PreventExtensions]] applied to the proxy object only returns true\n// if [[IsExtensible]] applied to the proxy object's target object is false.\ntry {\n  Object.preventExtensions(new Proxy({}, {\n    preventExtensions: function () {\n      passed = true;\n      return true;\n    }\n  }));\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"ownKeys\" handler", exec :
            "var proxied = {};\nvar passed = false;\nObject.keys(\n  new Proxy(proxied, {\n    ownKeys: function (t) {\n      passed = t === proxied; return [];\n    }\n  })\n);\nreturn passed;",
            }, Subtest { name : "\"ownKeys\" handler invariant", exec :
            "var passed = false;\nnew Proxy({},{});\n// The Type of each result List element is either String or Symbol.\ntry {\n  Object.keys(new Proxy({}, {\n    ownKeys: function () {\n      passed = true;\n      return [2];\n    }}));\n  return false;\n} catch(e) {}\n// The result List must contain the keys of all non-configurable own properties of the target object.\nvar proxied = {};\nObject.defineProperty(proxied, \"foo\", { value: 2, writable: true, enumerable: true });\ntry {\n  Object.keys(new Proxy(proxied, {\n    ownKeys: function () {\n      return [];\n    }}));\n  return false;\n} catch(e) {}\n// If the target object is not extensible, then the result List must contain all the keys\n// of the own properties of the target object and no other values.\ntry {\n  Object.keys(new Proxy(Object.preventExtensions({b:1}), {\n    ownKeys: function () {\n      return ['a'];\n    }}));\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"apply\" handler", exec :
            "var proxied = function(){};\nvar passed = false;\nvar host = {\n  method: new Proxy(proxied, {\n    apply: function (t, thisArg, args) {\n      passed = t === proxied && thisArg === host && args + \"\" === \"foo,bar\";\n    }\n  })\n};\nhost.method(\"foo\", \"bar\");\nreturn passed;",
            }, Subtest { name : "\"apply\" handler invariant", exec :
            "var passed = false;\nnew Proxy(function(){}, {\n    apply: function () { passed = true; }\n})();\n// A Proxy exotic object only has a [[Call]] internal method if the\n// initial value of its [[ProxyTarget]] internal slot is an object\n// that has a [[Call]] internal method.\ntry {\n  new Proxy({}, {\n    apply: function () {}\n  })();\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "\"construct\" handler", exec :
            "var proxied = function(){};\nvar passed = false;\nnew new Proxy(proxied, {\n  construct: function (t, args) {\n    passed = t === proxied && args + \"\" === \"foo,bar\";\n    return {};\n  }\n})(\"foo\",\"bar\");\nreturn passed;",
            }, Subtest { name : "\"construct\" handler invariants", exec :
            "var passed = false;\nnew Proxy({},{});\n// A Proxy exotic object only has a [[Construct]] internal method if the\n// initial value of its [[ProxyTarget]] internal slot is an object\n// that has a [[Construct]] internal method.\ntry {\n  new new Proxy({}, {\n    construct: function (t, args) {\n      return {};\n    }\n  })();\n  return false;\n} catch(e) {}\n// The result of [[Construct]] must be an Object.\ntry {\n  new new Proxy(function(){}, {\n    construct: function (t, args) {\n      passed = true;\n      return 5;\n    }\n  })();\n  return false;\n} catch(e) {}\nreturn passed;",
            }, Subtest { name : "Proxy.revocable", exec :
            "var obj = Proxy.revocable({}, { get: function() { return 5; } });\nvar passed = (obj.proxy.foo === 5);\nobj.revoke();\ntry {\n  obj.proxy.foo;\n} catch(e) {\n  passed &= e instanceof TypeError;\n}\nreturn passed;",
            }, Subtest { name : "Array.isArray support", exec :
            "return Array.isArray(new Proxy([], {}));", }, Subtest { name :
            "JSON.stringify support", exec :
            "return JSON.stringify(new Proxy(['foo'], {})) === '[\"foo\"]';", },
        ]
    }
}
