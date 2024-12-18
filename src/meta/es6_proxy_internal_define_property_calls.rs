use crate::{
    feature::{Meta, Subtest},
    features::Es6ProxyInternalDefinePropertyCalls,
};
impl Meta for Es6ProxyInternalDefinePropertyCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'defineProperty' calls"
    }

    fn key(&self) -> &'static str {
        "es6_proxy_internal_define_property_calls"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/defineProperty"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "[[Set]]",
                exec: "// [[Set]] -> [[DefineOwnProperty]]\nvar def = [];\nvar p = new Proxy({foo:1, bar:2}, { defineProperty: function(o, v, desc) { def.push(v); Object.defineProperty(o, v, desc); return true; }});\np.foo = 2; p.bar = 4;\nreturn def + '' === \"foo,bar\";",
            },
            Subtest {
                name: "SetIntegrityLevel",
                exec: "// SetIntegrityLevel -> DefinePropertyOrThrow -> [[DefineOwnProperty]]\nvar def = [];\nvar p = new Proxy({foo:1, bar:2}, { defineProperty: function(o, v, desc) { def.push(v); Object.defineProperty(o, v, desc); return true; }});\nObject.freeze(p);\nreturn def + '' === \"foo,bar\";",
            },
        ]
    }
}
