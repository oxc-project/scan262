use crate::feature::{Meta, Subtest};
use crate::features::Es6PrototypeOfBoundFunctions;
impl Meta for Es6PrototypeOfBoundFunctions {
    fn name(&self) -> &'static str {
        "prototype of bound functions"
    }
    fn key(&self) -> &'static str {
        "es6_prototype_of_bound_functions"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-boundfunctioncreate"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functions", exec :
            "function correctProtoBound(proto) {\n  var f = function(){};\n  if (Object.setPrototypeOf) {\n    Object.setPrototypeOf(f, proto);\n  } else {\n    f.__proto__ = proto;\n  }\n  var boundF = Function.prototype.bind.call(f, null);\n  return Object.getPrototypeOf(boundF) === proto;\n}\nreturn correctProtoBound(Function.prototype)\n  && correctProtoBound({})\n  && correctProtoBound(null);"
            }, Subtest { name : "generator functions", exec :
            "function correctProtoBound(proto) {\n  var f = function*(){};\n  if (Object.setPrototypeOf) {\n    Object.setPrototypeOf(f, proto);\n  } else {\n    f.__proto__ = proto;\n  }\n  var boundF = Function.prototype.bind.call(f, null);\n  return Object.getPrototypeOf(boundF) === proto;\n}\nreturn correctProtoBound(Function.prototype)\n  && correctProtoBound({})\n  && correctProtoBound(null);"
            }, Subtest { name : "arrow functions", exec :
            "function correctProtoBound(proto) {\n  var f = ()=>5;\n  if (Object.setPrototypeOf) {\n    Object.setPrototypeOf(f, proto);\n  } else {\n    f.__proto__ = proto;\n  }\n  var boundF = Function.prototype.bind.call(f, null);\n  return Object.getPrototypeOf(boundF) === proto;\n}\nreturn correctProtoBound(Function.prototype)\n  && correctProtoBound({})\n  && correctProtoBound(null);"
            }, Subtest { name : "classes", exec :
            "function correctProtoBound(proto) {\n  class C {}\n  if (Object.setPrototypeOf) {\n    Object.setPrototypeOf(C, proto);\n  } else {\n    C.__proto__ = proto;\n  }\n  var boundF = Function.prototype.bind.call(C, null);\n  return Object.getPrototypeOf(boundF) === proto;\n}\nreturn correctProtoBound(Function.prototype)\n  && correctProtoBound({})\n  && correctProtoBound(null);"
            }, Subtest { name : "subclasses", exec :
            "function correctProtoBound(superclass) {\n  class C extends superclass {\n    constructor() {\n      return Object.create(null);\n    }\n  }\n  var boundF = Function.prototype.bind.call(C, null);\n  return Object.getPrototypeOf(boundF) === Object.getPrototypeOf(C);\n}\nreturn correctProtoBound(function(){})\n  && correctProtoBound(Array)\n  && correctProtoBound(null);"
            },
        ]
    }
}
