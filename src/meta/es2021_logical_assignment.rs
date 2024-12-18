use crate::feature::{Meta, Subtest};
use crate::features::Es2021LogicalAssignment;
impl Meta for Es2021LogicalAssignment {
    fn name(&self) -> &'static str {
        "Logical Assignment"
    }
    fn key(&self) -> &'static str {
        "es2021_logical_assignment"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2021 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-logical-assignment/"
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
            Subtest { name : "||= basic support", exec :
            "let a;\nlet b = 0;\nlet c = 1;\na ||= 2;\nb ||= 2;\nc ||= 2;\nreturn a === 2 && b === 2 && c === 1;"
            }, Subtest { name : "||= short-circuiting behaviour", exec :
            "let a = 1;\nlet i = 1;\na ||= ++i;\nreturn a === 1 && i === 1;" }, Subtest {
            name : "||= setter not unecessarily invoked", exec :
            "let i = 1;\nvar obj = { get x() { return 1 }, set x(n) { i++; } };\nobj.x ||= 2;\nreturn i === 1;"
            }, Subtest { name : "&&= basic support", exec :
            "let a;\nlet b = 0;\nlet c = 1;\na &&= 2;\nb &&= 2;\nc &&= 2;\nreturn typeof a === 'undefined' && b === 0 && c === 2;"
            }, Subtest { name : "&&= short-circuiting behaviour", exec :
            "let a;\nlet i = 1;\na &&= ++i;\nreturn typeof a === 'undefined' && i === 1;"
            }, Subtest { name : "&&= setter not unecessarily invoked", exec :
            "let i = 1;\nvar obj = { get x() { return }, set x(n) { i++; } };\nobj.x &&= 2;\nreturn i === 1;"
            }, Subtest { name : "??= basic support", exec :
            "let a;\nlet b = 0;\nlet c = 1;\na ??= 2;\nb ??= 2;\nc ??= 2;\nreturn a === 2 && b === 0 && c === 1;"
            }, Subtest { name : "??= short-circuiting behaviour", exec :
            "let a = 1;\nlet i = 1;\na ??= ++i;\nreturn a === 1 && i === 1;" }, Subtest {
            name : "??= setter not unecessarily invoked", exec :
            "let i = 1;\nvar obj = { get x() { return 1 }, set x(n) { i++; } };\nobj.x ??= 2;\nreturn i === 1;"
            },
        ]
    }
}
