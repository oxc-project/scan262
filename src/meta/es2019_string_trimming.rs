use crate::feature::{Meta, Subtest};
use crate::features::Es2019StringTrimming;
impl Meta for Es2019StringTrimming {
    fn name(&self) -> &'static str {
        "string trimming"
    }
    fn key(&self) -> &'static str {
        "es2019_string_trimming"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2019 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-string-left-right-trim"
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
            Subtest {
                name: "String.prototype.trimLeft",
                exec: "return ' \\t \\n abc   \\t\\n'.trimLeft() === 'abc   \\t\\n';",
            },
            Subtest {
                name: "String.prototype.trimRight",
                exec: "return ' \\t \\n abc   \\t\\n'.trimRight() === ' \\t \\n abc';",
            },
            Subtest {
                name: "String.prototype.trimStart",
                exec: "return ' \\t \\n abc   \\t\\n'.trimStart() === 'abc   \\t\\n';",
            },
            Subtest {
                name: "String.prototype.trimEnd",
                exec: "return ' \\t \\n abc   \\t\\n'.trimEnd() === ' \\t \\n abc';",
            },
        ]
    }
}
