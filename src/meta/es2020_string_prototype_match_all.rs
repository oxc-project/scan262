use crate::feature::{Meta, Subtest};
use crate::features::Es2020StringPrototypeMatchAll;
impl Meta for Es2020StringPrototypeMatchAll {
    fn name(&self) -> &'static str {
        "String.prototype.matchAll"
    }
    fn key(&self) -> &'static str {
        "es2020_string_prototype_match_all"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2020 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/String.prototype.matchAll"
    }
    fn significance(&self) -> &'static str {
        "small"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/matchAll"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var iterator = '11a2bb'.matchAll(/(\\d)(\\D)/g);\nif(iterator[Symbol.iterator]() !== iterator)return false;\nvar a = '', b = '', c = '', step;\nwhile(!(step = iterator.next()).done) {\n  a += step.value[0];\n  b += step.value[1];\n  c += step.value[2];\n}\nreturn a === '1a2b'\n  && b === '12'\n  && c === 'ab';",
            }, Subtest { name : "throws on non-global regex", exec :
            "if (typeof String.prototype.matchAll !== 'function') return false;\ntry {\n  '11a2bb'.matchAll(/(\\d)(\\D)/);\n} catch (e) {\n  return true;\n}",
            },
        ]
    }
}
