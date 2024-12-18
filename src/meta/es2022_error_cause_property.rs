use crate::feature::{Meta, Subtest};
use crate::features::Es2022ErrorCauseProperty;
impl Meta for Es2022ErrorCauseProperty {
    fn name(&self) -> &'static str {
        "Error.cause property"
    }
    fn key(&self) -> &'static str {
        "es2022_error_cause_property"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-error-cause"
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
            Subtest { name : "Error has cause", exec :
            "var error = new Error('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "Error.prototype lacks cause", exec :
            "return !('cause' in Error.prototype);" }, Subtest { name :
            "EvalError has cause", exec :
            "var error = new EvalError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "EvalError.prototype lacks cause", exec :
            "return !('cause' in EvalError.prototype);" }, Subtest { name :
            "RangeError has cause", exec :
            "var error = new RangeError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "RangeError.prototype lacks cause", exec :
            "return !('cause' in RangeError.prototype);" }, Subtest { name :
            "ReferenceError has cause", exec :
            "var error = new ReferenceError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "ReferenceError.prototype lacks cause", exec :
            "return !('cause' in ReferenceError.prototype);" }, Subtest { name :
            "SyntaxError has cause", exec :
            "var error = new SyntaxError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "SyntaxError.prototype lacks cause", exec :
            "return !('cause' in SyntaxError.prototype);" }, Subtest { name :
            "TypeError has cause", exec :
            "var error = new TypeError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "TypeError.prototype lacks cause", exec :
            "return !('cause' in TypeError.prototype);" }, Subtest { name :
            "URIError has cause", exec :
            "var error = new URIError('error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "URIError.prototype lacks cause", exec :
            "return !('cause' in URIError.prototype);" }, Subtest { name :
            "AggregateError has cause", exec :
            "var error = new AggregateError([], 'error', { cause: 'cause' })\nreturn error.hasOwnProperty('cause') && error.cause === 'cause';"
            }, Subtest { name : "AggregateError.prototype lacks cause", exec :
            "return !('cause' in AggregateError.prototype);" },
        ]
    }
}
