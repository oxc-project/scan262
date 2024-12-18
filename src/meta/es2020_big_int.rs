use crate::feature::{Meta, Subtest};
use crate::features::Es2020BigInt;
impl Meta for Es2020BigInt {
    fn name(&self) -> &'static str {
        "BigInt"
    }
    fn key(&self) -> &'static str {
        "es2020_big_int"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-bigint"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/BigInt"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec : "return (1n + 2n) === 3n;" },
            Subtest { name : "constructor", exec : "return BigInt(\"3\") === 3n;" },
            Subtest { name : "BigInt.asUintN", exec :
            "return typeof BigInt.asUintN === 'function';" }, Subtest { name :
            "BigInt.asIntN", exec : "return typeof BigInt.asIntN === 'function';" },
            Subtest { name : "BigInt64Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new BigInt64Array(buffer);\nview[0] = 0x8000000000000000n;\nreturn view[0] === -0x8000000000000000n;"
            }, Subtest { name : "BigUint64Array", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new BigUint64Array(buffer);\nview[0] = 0x10000000000000000n;\nreturn view[0] === 0n;"
            }, Subtest { name : "DataView.prototype.getBigInt64", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setBigInt64(0, 1n);\nreturn view.getBigInt64(0) === 1n;"
            }, Subtest { name : "DataView.prototype.getBigUint64", exec :
            "var buffer = new ArrayBuffer(64);\nvar view = new DataView(buffer);\nview.setBigUint64(0, 1n);\nreturn view.getBigUint64(0) === 1n;"
            },
        ]
    }
}
