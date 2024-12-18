use crate::{
    feature::{Meta, Subtest},
    features::Es6ProxyInternalGetOwnPropertyDescriptorCalls,
};
impl Meta for Es6ProxyInternalGetOwnPropertyDescriptorCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'getOwnPropertyDescriptor' calls"
    }

    fn key(&self) -> &'static str {
        "es6_proxy_internal_get_own_property_descriptor_calls"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/getOwnPropertyDescriptor"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "[[Set]]",
                exec: "// [[Set]] -> [[GetOwnProperty]]\nvar gopd = [];\nvar p = new Proxy({},\n  { getOwnPropertyDescriptor: function(o, v) { gopd.push(v); return Object.getOwnPropertyDescriptor(o, v); }});\np.foo = 1; p.bar = 1;\nreturn gopd + '' === \"foo,bar\";",
            },
            Subtest {
                name: "Object.assign",
                exec: "// Object.assign -> [[GetOwnProperty]]\nvar gopd = [];\nvar p = new Proxy({foo:1, bar:2},\n  { getOwnPropertyDescriptor: function(o, v) { gopd.push(v); return Object.getOwnPropertyDescriptor(o, v); }});\nObject.assign({}, p);\nreturn gopd + '' === \"foo,bar\";",
            },
            Subtest {
                name: "Object.prototype.hasOwnProperty",
                exec: "// Object.prototype.hasOwnProperty -> HasOwnProperty -> [[GetOwnProperty]]\nvar gopd = [];\nvar p = new Proxy({foo:1, bar:2},\n  { getOwnPropertyDescriptor: function(o, v) { gopd.push(v); return Object.getOwnPropertyDescriptor(o, v); }});\np.hasOwnProperty('garply');\nreturn gopd + '' === \"garply\";",
            },
            Subtest {
                name: "Function.prototype.bind",
                exec: "// Function.prototype.bind -> HasOwnProperty -> [[GetOwnProperty]]\nvar gopd = [];\nvar p = new Proxy(Function(),\n  { getOwnPropertyDescriptor: function(o, v) { gopd.push(v); return Object.getOwnPropertyDescriptor(o, v); }});\np.bind();\nreturn gopd + '' === \"length\";",
            },
        ]
    }
}
