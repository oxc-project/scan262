use crate::feature::{Meta, Subtest};
use crate::features::Es2016ArrayPrototypeIncludes;
impl Meta for Es2016ArrayPrototypeIncludes {
    fn name(&self) -> &'static str {
        "Array.prototype.includes"
    }
    fn key(&self) -> &'static str {
        "es2016_array_prototype_includes"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 features"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-array.prototype.includes"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/includes"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "Array.prototype.includes", exec :
            "return [1, 2, 3].includes(1)\n  && ![1, 2, 3].includes(4)\n  && ![1, 2, 3].includes(1, 1)\n  && [NaN].includes(NaN);"
            }, Subtest { name : "Array.prototype.includes handles sparse arrays", exec :
            "return [,].includes()\n  && Array(1).includes();" }, Subtest { name :
            "Array.prototype.includes is generic", exec :
            "var passed = 0;\nreturn [].includes.call(\n  {\n    get \"0\"() {\n      passed = NaN;\n      return 'foo';\n    },\n    get \"11\"() {\n      passed += 1;\n      return 0;\n    },\n    get \"19\"() {\n      passed += 1;\n      return 'foo';\n    },\n    get \"21\"() {\n      passed = NaN;\n      return 'foo';\n    },\n    get length() {\n      passed += 1;\n      return 24;\n    }\n  },\n  'foo',\n  6\n) === true && passed === 3;"
            }, Subtest { name : "%TypedArray%.prototype.includes", exec :
            "return [\n  Int8Array,\n  Uint8Array,\n  Uint8ClampedArray,\n  Int16Array,\n  Uint16Array,\n  Int32Array,\n  Uint32Array,\n  Float32Array,\n  Float64Array\n].every(function (TypedArray) {\n  return new TypedArray([1, 2, 3]).includes(1)\n  && !new TypedArray([1, 2, 3]).includes(4)\n  && !new TypedArray([1, 2, 3]).includes(1, 1);\n});"
            },
        ]
    }
}
