use crate::{
    feature::{Meta, Subtest},
    features::EsnextRegExpEscaping,
};
impl Meta for EsnextRegExpEscaping {
    fn name(&self) -> &'static str {
        "RegExp Escaping"
    }

    fn key(&self) -> &'static str {
        "esnext_reg_exp_escaping"
    }

    fn target(&self) -> &'static str {
        "esnext"
    }

    fn category(&self) -> &'static str {
        "Stage 3"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regex-escaping"
    }

    fn significance(&self) -> &'static str {
        "medium"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "return RegExp.escape(\"The Quick Brown Fox\") === \"The\\\\ Quick\\\\ Brown\\\\ Fox\" &&\n  RegExp.escape(\"(*.*)\") === \"\\\\(\\\\*\\\\.\\\\*\\\\)\" &&\n  RegExp.escape(\"｡^･ｪ･^｡\") === \"｡\\\\^･ｪ･\\\\^｡\" &&\n  RegExp.escape(\"\\\\d \\\\D (?:)\") === \"\\\\\\\\d \\\\\\\\D \\\\(\\\\?\\\\:\\\\)\";"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
