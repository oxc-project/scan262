use crate::feature::{Meta, Subtest};
use crate::features::Es2023ChangeArrayByCopy;
impl Meta for Es2023ChangeArrayByCopy {
    fn name(&self) -> &'static str {
        "Change Array by copy"
    }
    fn key(&self) -> &'static str {
        "es2023_change_array_by_copy"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2023 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-change-array-by-copy"
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
            Subtest { name : "Array.prototype.toReversed()", exec :
            "var arr = [1, 2, 3];\nreturn arr.toReversed()[0] === 3 && arr[0] === 1;", },
            Subtest { name : "Array.prototype.toSorted()", exec :
            "var arr = ['C', 'A', 'B'];\nreturn arr.toSorted()[0] === 'A' && arr[0] === 'C';",
            }, Subtest { name : "Array.prototype.toSpliced()", exec :
            "var arr = ['A', 'C'];\nreturn arr.toSpliced(1, 0, 'B')[1] === 'B' && arr[1] === 'C';",
            }, Subtest { name : "Array.prototype.with()", exec :
            "var arr = ['A', 'X', 'C'];\nreturn arr.with(1, 'B')[1] === 'B' && arr[1] === 'X';",
            }, Subtest { name : "TypedArray.prototype.toReversed()", exec :
            "var arr = new Uint8Array([1, 2, 3]);\nreturn arr.toReversed()[0] == 3 && arr[0] == 1;",
            }, Subtest { name : "TypedArray.prototype.toSorted()", exec :
            "var arr = new Uint8Array([3, 1, 2]);\nreturn arr.toSorted()[0] == 1 && arr[0] == 3;",
            }, Subtest { name : "TypedArray.prototype.with()", exec :
            "var arr = new Uint8Array([1, 0, 2]);\nreturn arr.with(1, 2)[1] == 2 && arr[1] == 0;",
            },
        ]
    }
}
