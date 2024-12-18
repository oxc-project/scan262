use crate::{
    feature::{Meta, Subtest},
    features::Es6ProxyInternalOwnKeysCalls,
};
impl Meta for Es6ProxyInternalOwnKeysCalls {
    fn name(&self) -> &'static str {
        "Proxy, internal 'ownKeys' calls"
    }

    fn key(&self) -> &'static str {
        "es6_proxy_internal_own_keys_calls"
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
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/ownKeys"
    }

    fn exec(&self) -> &'static str {
        ""
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest {
                name: "SetIntegrityLevel",
                exec: "// SetIntegrityLevel -> [[OwnPropertyKeys]]\nvar ownKeysCalled = 0;\nvar p = new Proxy({}, { ownKeys: function(o) { ownKeysCalled++; return Object.keys(o); }});\nObject.freeze(p);\nreturn ownKeysCalled === 1;",
            },
            Subtest {
                name: "TestIntegrityLevel",
                exec: "// TestIntegrityLevel -> [[OwnPropertyKeys]]\nvar ownKeysCalled = 0;\nvar p = new Proxy(Object.preventExtensions({}), { ownKeys: function(o) { ownKeysCalled++; return Object.keys(o); }});\nObject.isFrozen(p);\nreturn ownKeysCalled === 1;",
            },
            Subtest {
                name: "SerializeJSONObject",
                exec: "// SerializeJSONObject -> EnumerableOwnNames -> [[OwnPropertyKeys]]\nvar ownKeysCalled = 0;\nvar p = new Proxy({}, { ownKeys: function(o) { ownKeysCalled++; return Object.keys(o); }});\nJSON.stringify({a:p,b:p});\nreturn ownKeysCalled === 2;",
            },
        ]
    }
}
