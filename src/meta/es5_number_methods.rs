use crate::feature::{Meta, Subtest};
use crate::features::Es5NumberMethods;
impl Meta for Es5NumberMethods {
    fn name(&self) -> &'static str {
        "Number methods"
    }
    fn key(&self) -> &'static str {
        "es5_number_methods"
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
            Subtest { name : "Number.prototype.toExponential rounds properly", exec :
            "function () {\nreturn (-6.9e-11).toExponential(4) === '-6.9000e-11' // Edge <= 17\n  && (25).toExponential(0) === '3e+1' // FF <= 86\n  && (1.255).toExponential(2) === '1.25e+0'; // IE <= 11 && Edge <= 14\n    }"
            }, Subtest { name :
            "Number.prototype.toExponential throws on ±Infinity fractionDigits", exec :
            "function () {\ntry {\n  (1).toExponential(Infinity);\n  (1).toExponential(-Infinity);\n  return false;\n} catch (e) {\n  return true;\n}\n    }"
            }, Subtest { name :
            "Number.prototype.toExponential does not throw on edge cases", exec :
            "function () {\ntry {\n  NaN.toExponential(Infinity);\n  Infinity.toExponential(Infinity);\n  return true;\n} catch (e) {\n  return false;\n}\n    }"
            },
        ]
    }
}
