use crate::feature::{Meta, Subtest};
use crate::features::Es2019JsonSuperset;
impl Meta for Es2019JsonSuperset {
    fn name(&self) -> &'static str {
        "JSON superset"
    }
    fn key(&self) -> &'static str {
        "es2019_json_superset"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 misc"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-json-superset"
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
            Subtest {
                name: "LINE SEPARATOR can appear in string literals",
                exec: "return eval(\"'\\u2028'\") === \"\\u2028\";",
            },
            Subtest {
                name: "PARAGRAPH SEPARATOR can appear in string literals",
                exec: "return eval(\"'\\u2029'\") === \"\\u2029\";",
            },
        ]
    }
}
