use crate::feature::{Meta, Subtest};
use crate::features::EsnextMapPrototypeUpsert;
impl Meta for EsnextMapPrototypeUpsert {
    fn name(&self) -> &'static str {
        "Map.prototype.upsert"
    }
    fn key(&self) -> &'static str {
        "esnext_map_prototype_upsert"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 2"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-upsert"
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
            Subtest { name : "Map.prototype.upsert", exec :
            "const map = new Map([['a', 1]]);\nif (map.upsert('a', it => 2, () => 3) !== 2) return false;\nif (map.upsert('b', it => 2, () => 3) !== 3) return false;\nreturn Array.from(map).join() === 'a,2,b,3';",
            }, Subtest { name : "WeakMap.prototype.upsert", exec :
            "const a = {}, b = {};\nconst map = new WeakMap([[a, 1]]);\nif (map.upsert(a, it => 2, () => 3) !== 2) return false;\nif (map.upsert(b, it => 2, () => 3) !== 3) return false;\nreturn map.get(a) === 2 && map.get(b) === 3;",
            },
        ]
    }
}
