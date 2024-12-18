use crate::feature::{Meta, Subtest};
use crate::features::Es2019ArrayPrototypeFlatFlatMap;
impl Meta for Es2019ArrayPrototypeFlatFlatMap {
    fn name(&self) -> &'static str {
        "Array.prototype.{flat, flatMap}"
    }
    fn key(&self) -> &'static str {
        "es2019_array_prototype_flat_flat_map"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 features"
    }
    fn spec(&self) -> &'static str {
        "https://tc39.github.io/proposal-flatMap/"
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
            Subtest { name : "Array.prototype.flat", exec :
            "return [1, [2, 3], [4, [5, 6]]].flat().join('') === '12345,6';", }, Subtest
            { name : "Array.prototype.flatMap", exec :
            "return [{a: 1, b: 2}, {a: 3, b: 4}].flatMap(function (it) {\n  return [it.a, it.b];\n}).join('') === '1234';",
            }, Subtest { name : "flat and flatMap in Array.prototype[@@unscopables]",
            exec :
            "return Array.prototype[Symbol.unscopables].flat\n  && Array.prototype[Symbol.unscopables].flatMap;",
            },
        ]
    }
}
