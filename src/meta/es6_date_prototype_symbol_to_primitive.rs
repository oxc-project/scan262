use crate::{
    feature::{Meta, Subtest},
    features::Es6DatePrototypeSymbolToPrimitive,
};
impl Meta for Es6DatePrototypeSymbolToPrimitive {
    fn name(&self) -> &'static str {
        "Date.prototype[Symbol.toPrimitive]"
    }

    fn key(&self) -> &'static str {
        "es6_date_prototype_symbol_to_primitive"
    }

    fn target(&self) -> &'static str {
        "es6"
    }

    fn category(&self) -> &'static str {
        "built-in extensions"
    }

    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-date.prototype-@@toprimitive"
    }

    fn significance(&self) -> &'static str {
        "tiny"
    }

    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date/@@toPrimitive"
    }

    fn exec(&self) -> &'static str {
        "var tp = Date.prototype[Symbol.toPrimitive];\nreturn tp.call(Object(2), \"number\") === 2\n  && tp.call(Object(2), \"string\") === \"2\"\n  && tp.call(Object(2), \"default\") === \"2\";"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
