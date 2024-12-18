use crate::feature::{Meta, Subtest};
use crate::features::Es2018ObjectRestSpreadProperties;
impl Meta for Es2018ObjectRestSpreadProperties {
    fn name(&self) -> &'static str {
        "object rest/spread properties"
    }
    fn key(&self) -> &'static str {
        "es2018_object_rest_spread_properties"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2018 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-object-rest-spread"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "object rest properties", exec :
            "var { a, ...rest } = { a: 1, b: 2, c: 3 };\nreturn a === 1 && rest.a === void undefined && rest.b === 2 && rest.c === 3;"
            }, Subtest { name : "object spread properties", exec :
            "var spread = { b: 2, c: 3 };\nvar O = { a: 1, ...spread };\nreturn O !== spread && (O.a + O.b + O.c) === 6;"
            },
        ]
    }
}
