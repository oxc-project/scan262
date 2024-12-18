use crate::feature::{Meta, Subtest};
use crate::features::Es2019SymbolPrototypeDescription;
impl Meta for Es2019SymbolPrototypeDescription {
    fn name(&self) -> &'static str {
        "Symbol.prototype.description"
    }
    fn key(&self) -> &'static str {
        "es2019_symbol_prototype_description"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-Symbol-description"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Symbol/description"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic", exec :
            "return Symbol('foo').description === 'foo';" }, Subtest { name :
            "empty description", exec : "return Symbol('').description === '';" },
            Subtest { name : "undefined description", exec :
            "return Symbol.prototype.hasOwnProperty('description')\n  && Symbol().description === void undefined;"
            },
        ]
    }
}
