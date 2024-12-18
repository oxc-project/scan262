use crate::feature::{Meta, Subtest};
use crate::features::Es6ArrayIsSubclassable;
impl Meta for Es6ArrayIsSubclassable {
    fn name(&self) -> &'static str {
        "Array is subclassable"
    }
    fn key(&self) -> &'static str {
        "es6_array_is_subclassable"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "subclassing"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-array-constructor"
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
            Subtest { name : "length property (accessing)", exec :
            "class C extends Array {}\nvar c = new C();\nvar len1 = c.length;\nc[2] = 'foo';\nvar len2 = c.length;\nreturn len1 === 0 && len2 === 3;"
            }, Subtest { name : "length property (setting)", exec :
            "class C extends Array {}\nvar c = new C();\nc[2] = 'foo';\nc.length = 1;\nreturn c.length === 1 && !(2 in c);"
            }, Subtest { name : "correct prototype chain", exec :
            "class C extends Array {}\nvar c = new C();\nreturn c instanceof C && c instanceof Array && Object.getPrototypeOf(C) === Array;"
            }, Subtest { name : "Array.isArray support", exec :
            "class C extends Array {}\nreturn Array.isArray(new C());" }, Subtest { name
            : "Array.prototype.concat", exec :
            "class C extends Array {}\nvar c = new C();\nreturn c.concat(1) instanceof C;"
            }, Subtest { name : "Array.prototype.filter", exec :
            "class C extends Array {}\nvar c = new C();\nreturn c.filter(Boolean) instanceof C;"
            }, Subtest { name : "Array.prototype.map", exec :
            "class C extends Array {}\nvar c = new C();\nreturn c.map(Boolean) instanceof C;"
            }, Subtest { name : "Array.prototype.slice", exec :
            "class C extends Array {}\nvar c = new C();\nc.push(2,4,6);\nreturn c.slice(1,2) instanceof C;"
            }, Subtest { name : "Array.prototype.splice", exec :
            "class C extends Array {}\nvar c = new C();\nc.push(2,4,6);\nreturn c.splice(1,2) instanceof C;"
            }, Subtest { name : "Array.from", exec :
            "class C extends Array {}\nreturn C.from({ length: 0 }) instanceof C;" },
            Subtest { name : "Array.of", exec :
            "class C extends Array {}\nreturn C.of(0) instanceof C;" },
        ]
    }
}
