use crate::feature::{Meta, Subtest};
use crate::features::Es6RegExpPrototypeProperties;
impl Meta for Es6RegExpPrototypeProperties {
    fn name(&self) -> &'static str {
        "RegExp.prototype properties"
    }
    fn key(&self) -> &'static str {
        "es6_reg_exp_prototype_properties"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "built-in extensions"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-regexp.prototype"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/RegExp/prototype"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "RegExp.prototype.flags", exec :
            "return /./igm.flags === \"gim\" && /./.flags === \"\";", }, Subtest { name :
            "RegExp.prototype[Symbol.match]", exec :
            "return typeof RegExp.prototype[Symbol.match] === 'function';", }, Subtest {
            name : "RegExp.prototype[Symbol.replace]", exec :
            "return typeof RegExp.prototype[Symbol.replace] === 'function';", }, Subtest
            { name : "RegExp.prototype[Symbol.split]", exec :
            "return typeof RegExp.prototype[Symbol.split] === 'function';", }, Subtest {
            name : "RegExp.prototype[Symbol.search]", exec :
            "return typeof RegExp.prototype[Symbol.search] === 'function';", }, Subtest {
            name : "RegExp[Symbol.species]", exec :
            "var prop = Object.getOwnPropertyDescriptor(RegExp, Symbol.species);\nreturn 'get' in prop && RegExp[Symbol.species] === RegExp;",
            },
        ]
    }
}
