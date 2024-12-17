use crate::feature::{Meta, Subtest};
use crate::features::Es2025SetMethods;
impl Meta for Es2025SetMethods {
    fn name(&self) -> &'static str {
        "Set methods"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2025 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-set-methods"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set#instance_methods"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Set.prototype.intersection()", exec :
            "var set = new Set([1, 2, 3]).intersection(new Set([2, 3, 4]));\nreturn set.size === 2\n  && set.has(2)\n  && set.has(3);",
            }, Subtest { name : "Set.prototype.union()", exec :
            "var set = new Set([1, 2]).union(new Set([2, 3]));\nreturn set.size === 3\n  && set.has(1)\n  && set.has(2)\n  && set.has(3);",
            }, Subtest { name : "Set.prototype.difference()", exec :
            "var set = new Set([1, 2, 3]).difference(new Set([3, 4]));\nreturn set.size === 2\n  && set.has(1)\n  && set.has(2);",
            }, Subtest { name : "Set.prototype.symmetricDifference()", exec :
            "var set = new Set([1, 2]).symmetricDifference(new Set([2, 3]));\nreturn set.size === 2\n  && set.has(1)\n  && set.has(3);",
            }, Subtest { name : "Set.prototype.isDisjointFrom()", exec :
            "return new Set([1, 2, 3]).isDisjointFrom(new Set([4, 5, 6]));", }, Subtest {
            name : "Set.prototype.isSubsetOf()", exec :
            "return new Set([1, 2, 3]).isSubsetOf(new Set([5, 4, 3, 2, 1]));", }, Subtest
            { name : "Set.prototype.isSupersetOf()", exec :
            "return new Set([5, 4, 3, 2, 1]).isSupersetOf(new Set([1, 2, 3]));", },
        ]
    }
}
