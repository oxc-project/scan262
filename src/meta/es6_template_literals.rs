use crate::feature::{Meta, Subtest};
use crate::features::Es6TemplateLiterals;
impl Meta for Es6TemplateLiterals {
    fn name(&self) -> &'static str {
        "template literals"
    }
    fn key(&self) -> &'static str {
        "es6_template_literals"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "syntax"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-template-literals"
    }
    fn significance(&self) -> &'static str {
        "large"
    }
    fn mdn(&self) -> &'static str {
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals"
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "basic functionality", exec :
            "var a = \"ba\", b = \"QUX\";\nreturn `foo bar\n${a + \"z\"} ${b.toLowerCase()}` === \"foo bar\\nbaz qux\";",
            }, Subtest { name : "toString conversion", exec :
            "var a = {\n  toString: function() { return \"foo\"; },\n  valueOf: function() { return \"bar\"; }\n};\nreturn `${a}` === \"foo\";",
            }, Subtest { name : "tagged template literals", exec :
            "var called = false;\nfunction fn(parts, a, b) {\n  called = true;\n  return parts instanceof Array &&\n    parts[0]     === \"foo\"      &&\n    parts[1]     === \"bar\\n\"    &&\n    parts.raw[0] === \"foo\"      &&\n    parts.raw[1] === \"bar\\\\n\"   &&\n    a === 123                   &&\n    b === 456;\n}\nreturn fn `foo${123}bar\\n${456}` && called;",
            }, Subtest { name : "passed array is frozen", exec :
            "return (function(parts) {\n  return Object.isFrozen(parts) && Object.isFrozen(parts.raw);\n}) `foo${0}bar${0}baz`;",
            }, Subtest { name : "line break normalisation", exec :
            "var cr   = eval(\"`x\" + String.fromCharCode(13)    + \"y`\");\nvar lf   = eval(\"`x\" + String.fromCharCode(10)    + \"y`\");\nvar crlf = eval(\"`x\" + String.fromCharCode(13,10) + \"y`\");\n\nreturn cr.length === 3 && lf.length === 3 && crlf.length === 3\n  && cr[1] === lf[1] && lf[1] === crlf[1] && crlf[1] === '\\n';",
            }, Subtest { name : "TemplateStrings call site caching", exec :
            "// TemplateStrings caching was changed from per-contents to\n// per-call-site.\n// https://github.com/tc39/ecma262/pull/890\nfunction strings(array) {\n  return array;\n}\nfunction getStrings() {\n  return strings`foo`;\n}\nvar original = getStrings();\nvar other = strings`foo`;\nreturn original === getStrings() && original !== other;",
            }, Subtest { name : "TemplateStrings permanent caching", exec :
            "function strings(array) {\n  return array;\n}\nfunction getStrings() {\n  return strings`foo`;\n}\nvar original = getStrings();\nvar newed = new getStrings();\nreturn original === getStrings() && original === newed;",
            },
        ]
    }
}
