use crate::{
    feature::{Meta, Subtest},
    features::Es2024ArrayGrouping,
};
impl Meta for Es2024ArrayGrouping {
    fn name(&self) -> &'static str {
        "Array Grouping"
    }

    fn key(&self) -> &'static str {
        "es2024_array_grouping"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2024 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-array-grouping"
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
                name: "Object.groupBy()",
                exec: "const array = [1, 2, 3, 4];\nconst obj = Object.groupBy(array, (num, index) => {\n  return num % 2 === 0 ? 'even': 'odd';\n});\nreturn !('toString' in obj) && obj.even[0] == 2 && obj.odd[0] == 1;",
            },
            Subtest {
                name: "Map.groupBy()",
                exec: "const array = [1, 2, 3, 4];\nconst odd  = { odd: true };\nconst even = { even: true };\nconst map = Map.groupBy(array, (num, index) => {\n  return num % 2 === 0 ? even: odd;\n});\nreturn map instanceof Map && map.get(even)[0] === 2 && map.get(odd)[0] === 1;",
            },
        ]
    }
}
