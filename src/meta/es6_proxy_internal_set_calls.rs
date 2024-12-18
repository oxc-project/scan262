use crate::feature::{Meta, Subtest};
use crate::features::Es6ProxyInternalSetCalls;
impl Meta for Es6ProxyInternalSetCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'set' calls"
    }
    fn key(&self) -> &'static str {
        "es6_proxy_internal_set_calls"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/set"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Object.assign", exec :
            "// Object.assign -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy({}, { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\nObject.assign(p, { foo: 1, bar: 2 });\nreturn set + '' === \"foo,bar\";"
            }, Subtest { name : "Array.from", exec :
            "// Array.from -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy({}, { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\nArray.from.call(function(){ return p; }, {length:2, 0:1, 1:2});\nreturn set + '' === \"length\";"
            }, Subtest { name : "Array.of", exec :
            "// Array.from -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy({}, { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\nArray.of.call(function(){ return p; }, 1, 2, 3);\nreturn set + '' === \"length\";"
            }, Subtest { name : "Array.prototype.copyWithin", exec :
            "// Array.prototype.copyWithin -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([1,2,3,4,5,6], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.copyWithin(0, 3);\nreturn set + '' === \"0,1,2\";"
            }, Subtest { name : "Array.prototype.fill", exec :
            "// Array.prototype.fill -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([1,2,3,4,5,6], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.fill(0, 3);\nreturn set + '' === \"3,4,5\";"
            }, Subtest { name : "Array.prototype.pop", exec :
            "// Array.prototype.pop -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.pop();\nreturn set + '' === \"length\";"
            }, Subtest { name : "Array.prototype.push", exec :
            "// Array.prototype.push -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.push(0,0,0);\nreturn set + '' === \"0,1,2,length\";"
            }, Subtest { name : "Array.prototype.reverse", exec :
            "// Array.prototype.reverse -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([0,0,0,,], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.reverse();\nreturn set + '' === \"3,1,2\";"
            }, Subtest { name : "Array.prototype.shift", exec :
            "// Array.prototype.shift -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([0,0,,0], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.shift();\nreturn set + '' === \"0,2,length\";"
            }, Subtest { name : "Array.prototype.splice", exec :
            "// Array.prototype.splice -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([1,2,3], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.splice(1,0,0);\nreturn set + '' === \"3,2,1,length\";"
            }, Subtest { name : "Array.prototype.unshift", exec :
            "// Array.prototype.unshift -> Set -> [[Set]]\nvar set = [];\nvar p = new Proxy([0,0,,0], { set: function(o, k, v) { set.push(k); o[k] = v; return true; }});\np.unshift(0,1);\nreturn set + '' === \"5,3,2,0,1,length\";"
            },
        ]
    }
}
