use crate::feature::{Meta, Subtest};
use crate::features::EsnextThrowExpressions;
impl Meta for EsnextThrowExpressions {
    fn name(&self) -> &'static str {
        "throw expressions"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 2"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-throw-expressions"
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
            Subtest { name : "logical", exec :
            "var a, b;\ntry {\n  a = 19 || throw 77;\n  b = 88 && throw 23;\n} catch (e) {\n  return a + e === 42;\n}",
            }, Subtest { name : "parameter initializers", exec :
            "function fn (arg = throw 42) {\n  return arg;\n}\n\nif (fn(21) !== 21) return false;\n\ntry {\n  fn();\n} catch (e) {\n  return e === 42;\n}",
            }, Subtest { name : "arrow function bodies", exec :
            "var fn = () => throw 42;\ntry {\n  fn();\n} catch (e) {\n  return e === 42;\n}",
            }, Subtest { name : "conditionals", exec :
            "true ? 42 : throw 21;\ntry {\n  false ? 42 : throw 21;\n} catch (e) {\n  return e === 21;\n}",
            },
        ]
    }
}
