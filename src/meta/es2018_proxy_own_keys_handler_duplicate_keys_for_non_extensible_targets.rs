use crate::{
    feature::{Meta, Subtest},
    features::Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets,
};
impl Meta for Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets {
    fn name(&self) -> &'static str {
        "Proxy \"ownKeys\" handler, duplicate keys for non-extensible targets"
    }

    fn key(&self) -> &'static str {
        "es2018_proxy_own_keys_handler_duplicate_keys_for_non_extensible_targets"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2018 misc"
    }

    fn spec(&self) -> &'static str {
        "https://tc39.es/ecma262/#sec-proxy-object-internal-methods-and-internal-slots-ownpropertykeys"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Proxy/handler/ownKeys"
    }

    fn exec(&self) -> &'static str {
        "var p = new Proxy({}, {\n  ownKeys() {\n    return [\"a\", \"a\"];\n  }\n});\ntry {\n  Object.keys(p);\n} catch (e) {\n  return e instanceof TypeError;\n}\nreturn false;"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
