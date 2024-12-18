use crate::feature::{Meta, Subtest};
use crate::features::Es2017ArgumentsCallerRemoved;
impl Meta for Es2017ArgumentsCallerRemoved {
    fn name(&self) -> &'static str {
        "arguments.caller removed"
    }
    fn key(&self) -> &'static str {
        "es2017_arguments_caller_removed"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2017 misc"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/ecma262/pull/689"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/arguments/caller"
    }
    fn exec(&self) -> &'static str {
        "return (function () {\n  'use strict';\n  return !Object.getOwnPropertyDescriptor(arguments, 'caller');\n})();"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
