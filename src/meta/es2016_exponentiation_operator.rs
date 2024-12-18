use crate::feature::{Meta, Subtest};
use crate::features::Es2016ExponentiationOperator;
impl Meta for Es2016ExponentiationOperator {
    fn name(&self) -> &'static str {
        "exponentiation (**) operator"
    }
    fn key(&self) -> &'static str {
        "es2016_exponentiation_operator"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2016 features"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/7.0/index.html#sec-exp-operator"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Arithmetic_Operators#Exponentiation_(**)"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic support", exec :
            "return 2 ** 3 === 8 && -(5 ** 2) === -25 && (-5) ** 2 === 25;", }, Subtest {
            name : "assignment", exec : "var a = 2; a **= 3; return a === 8;", }, Subtest
            { name : "early syntax error for unary negation without parens", exec :
            "if (2 ** 3 !== 8) { return false; }\ntry {\n  function (\"-5 ** 2\")();\n} catch (e) {\n  return true;\n}",
            },
        ]
    }
}
