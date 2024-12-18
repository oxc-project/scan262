use crate::feature::{Meta, Subtest};
use crate::features::Es2022AtMethodOnTheBuiltInIndexables;
impl Meta for Es2022AtMethodOnTheBuiltInIndexables {
    fn name(&self) -> &'static str {
        ".at() method on the built-in indexables"
    }
    fn key(&self) -> &'static str {
        "es2022_at_method_on_the_built_in_indexables"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-relative-indexing-method/"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Array.prototype.at()", exec :
            "var arr = [1, 2, 3];\nreturn arr.at(0) === 1\n  && arr.at(-3) === 1\n  && arr.at(1) === 2\n  && arr.at(-2) === 2\n  && arr.at(2) === 3\n  && arr.at(-1) === 3\n  && arr.at(3) === undefined\n  && arr.at(-4) === undefined;",
            }, Subtest { name : "String.prototype.at()", exec :
            "var str = 'abc';\nreturn str.at(0) === 'a'\n  && str.at(-3) === 'a'\n  && str.at(1) === 'b'\n  && str.at(-2) === 'b'\n  && str.at(2) === 'c'\n  && str.at(-1) === 'c'\n  && str.at(3) === undefined\n  && str.at(-4) === undefined;",
            }, Subtest { name : "%TypedArray%.prototype.at()", exec :
            "return [\n  'Int8Array',\n  'Uint8Array',\n  'Uint8ClampedArray',\n  'Int16Array',\n  'Uint16Array',\n  'Int32Array',\n  'Uint32Array',\n  'Float32Array',\n  'Float64Array'\n].every(function (TypedArray) {\n  var Constructor = globalThis[TypedArray];\n  if (typeof Constructor !== 'function') {\n    return false;\n  }\n  var arr = new Constructor([1, 2, 3]);\n  return arr.at(0) === 1\n    && arr.at(-3) === 1\n    && arr.at(1) === 2\n    && arr.at(-2) === 2\n    && arr.at(2) === 3\n    && arr.at(-1) === 3\n    && arr.at(3) === undefined\n    && arr.at(-4) === undefined;\n});",
            },
        ]
    }
}
