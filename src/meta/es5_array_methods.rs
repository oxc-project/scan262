use crate::{
    feature::{Meta, Subtest},
    features::Es5ArrayMethods,
};
impl Meta for Es5ArrayMethods {
    fn name(&self) -> &'static str {
        "Array methods"
    }

    fn key(&self) -> &'static str {
        "es5_array_methods"
    }

    fn target(&self) -> &'static str {
        "es5"
    }

    fn category(&self) -> &'static str {
        ""
    }

    fn spec(&self) -> &'static str {
        ""
    }

    fn significance(&self) -> &'static str {
        "large"
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
                name: "Array.isArray",
                exec: "function () {\nreturn typeof Array.isArray === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.indexOf",
                exec: "function () {\nreturn typeof Array.prototype.indexOf === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.lastIndexOf",
                exec: "function () {\nreturn typeof Array.prototype.lastIndexOf === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.every",
                exec: "function () {\nreturn typeof Array.prototype.every === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.some",
                exec: "function () {\nreturn typeof Array.prototype.some === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.forEach",
                exec: "function () {\nreturn typeof Array.prototype.forEach === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.map",
                exec: "function () {\nreturn typeof Array.prototype.map === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.filter",
                exec: "function () {\nreturn typeof Array.prototype.filter === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.reduce",
                exec: "function () {\nreturn typeof Array.prototype.reduce === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.reduceRight",
                exec: "function () {\nreturn typeof Array.prototype.reduceRight === 'function';\n      }",
            },
            Subtest {
                name: "Array.prototype.sort: compareFn must be function or undefined",
                exec: "function () {\ntry {\n  [1,2].sort(null);\n  return false;\n} catch (enull) {}\ntry {\n  [1,2].sort(true);\n  return false;\n} catch (etrue) {}\ntry {\n  [1,2].sort({});\n  return false;\n} catch (eobj) {}\ntry {\n  [1,2].sort([]);\n  return false;\n} catch (earr) {}\ntry {\n  [1,2].sort(/a/g);\n  return false;\n} catch (eregex) {}\nreturn true;\n      }",
            },
            Subtest {
                name: "Array.prototype.sort: compareFn may be explicit undefined",
                exec: "function () {\ntry {\n  var arr = [2, 1];\n  return arr.sort(undefined) === arr && arr[0] === 1 && arr[1] === 2;\n} catch (e) {\n  return false;\n}\n      }",
            },
            Subtest {
                name: "Array.prototype.unshift: [].unshift(0) returns the unshifted count",
                exec: "function () {\nreturn [].unshift(0) === 1;\n      }",
            },
        ]
    }
}
