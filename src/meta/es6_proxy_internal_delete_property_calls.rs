use crate::feature::{Meta, Subtest};
use crate::features::Es6ProxyInternalDeletePropertyCalls;
impl Meta for Es6ProxyInternalDeletePropertyCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'deleteProperty' calls"
    }
    fn key(&self) -> &'static str {
        "es6_proxy_internal_delete_property_calls"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-proxy-object-internal-methods-and-internal-slots"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/deleteProperty"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Array.prototype.copyWithin", exec :
            "// Array.prototype.copyWithin -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,0,0,,,,], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.copyWithin(0,3);\nreturn del + '' === \"0,1,2\";"
            }, Subtest { name : "Array.prototype.pop", exec :
            "// Array.prototype.pop -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,0,0], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.pop();\nreturn del + '' === \"2\";"
            }, Subtest { name : "Array.prototype.reverse", exec :
            "// Array.prototype.reverse -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,,2,,4,,], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.reverse();\nreturn del + '' === \"0,4,2\";"
            }, Subtest { name : "Array.prototype.shift", exec :
            "// Array.prototype.shift -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,,0,,0,0], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.shift();\nreturn del + '' === \"0,2,5\";"
            }, Subtest { name : "Array.prototype.splice", exec :
            "// Array.prototype.splice -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,0,0,0,,0], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.splice(2,2,0);\nreturn del + '' === \"3,5\";"
            }, Subtest { name : "Array.prototype.unshift", exec :
            "// Array.prototype.unshift -> DeletePropertyOrThrow -> [[Delete]]\nvar del = [];\nvar p = new Proxy([0,0,,0,,0], { deleteProperty: function(o, v) { del.push(v); return delete o[v]; }});\np.unshift(0);\nreturn del + '' === \"5,3\";"
            },
        ]
    }
}
