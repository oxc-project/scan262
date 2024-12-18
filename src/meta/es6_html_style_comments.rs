use crate::feature::{Meta, Subtest};
use crate::features::Es6HtmlStyleComments;
impl Meta for Es6HtmlStyleComments {
    fn name(&self) -> &'static str {
        "HTML-style comments"
    }
    fn key(&self) -> &'static str {
        "es6_html_style_comments"
    }
    fn target(&self) -> &'static str {
        "es6"
    }
    fn category(&self) -> &'static str {
        "annex b"
    }
    fn spec(&self) -> &'static str {
        "http://www.ecma-international.org/ecma-262/6.0/#sec-html-like-comments"
    }
    fn significance(&self) -> &'static str {
        "tiny"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        "--> A comment\n<!-- Another comment\nvar a = 3; <!-- Another comment\nreturn a === 3;"
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
