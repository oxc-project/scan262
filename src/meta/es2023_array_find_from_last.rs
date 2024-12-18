use crate::feature::{Meta, Subtest};
use crate::features::Es2023ArrayFindFromLast;
impl Meta for Es2023ArrayFindFromLast {
    fn name(&self) -> &'static str {
        "Array find from last"
    }
    fn key(&self) -> &'static str {
        "es2023_array_find_from_last"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2023 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-array-find-from-last"
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
            Subtest { name : "Array.prototype.findLast", exec :
            "var arr = [{ x: 1 }, { x: 2 }, { x: 1 }, { x: 2 }];\nreturn arr.findLast(function (o) { return o.x === 1; }) === arr[2];"
            }, Subtest { name : "Array.prototype.findLastIndex", exec :
            "var arr = [{ x: 1 }, { x: 2 }, { x: 1 }, { x: 2 }];\nreturn arr.findLastIndex(function (o) { return o.x === 1; }) === 2;"
            },
        ]
    }
}
