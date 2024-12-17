use crate::feature::{Meta, Subtest};
use crate::features::Es2016ProxyInternalCallsArrayPrototypeIncludes;
impl Meta for Es2016ProxyInternalCallsArrayPrototypeIncludes {
    fn name(&self) -> &'static str {
        "Proxy internal calls, Array.prototype.includes"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-array.prototype.includes"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "// Array.prototype.includes -> Get -> [[Get]]\nvar get = [];\nvar p = new Proxy({length: 3, 0: '', 1: '', 2: '', 3: ''}, { get: function (o, k) { get.push(k); return o[k]; }});\nArray.prototype.includes.call(p, {});\nif (get + '' !== \"length,0,1,2\") return;\n\nget = [];\np = new Proxy({length: 4, 0: NaN, 1: '', 2: NaN, 3: ''}, { get: function (o, k) { get.push(k); return o[k]; }});\nArray.prototype.includes.call(p, NaN, 1);\nreturn (get + '' === \"length,1,2\");"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
