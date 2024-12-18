use crate::feature::{Meta, Subtest};
use crate::features::Es2016ProxyEnumerateHandlerRemoved;
impl Meta for Es2016ProxyEnumerateHandlerRemoved {
    fn name(&self) -> &'static str {
        "Proxy, \"enumerate\" handler removed"
    }
    fn key(&self) -> &'static str {
        "es2016_proxy_enumerate_handler_removed"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 misc"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-proxy-objects"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/enumerate"
    }
    fn exec(&self) -> &'static str {
        "var passed = true;\nvar proxy = new Proxy({}, {\n  enumerate: function () {\n    passed = false;\n  }\n});\nfor (var key in proxy); // Should not throw, nor execute the 'enumerate' method.\nreturn passed;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
