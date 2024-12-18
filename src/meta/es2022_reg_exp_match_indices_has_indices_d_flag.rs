use crate::{
    feature::{Meta, Subtest},
    features::Es2022RegExpMatchIndicesHasIndicesDFlag,
};
impl Meta for Es2022RegExpMatchIndicesHasIndicesDFlag {
    fn name(&self) -> &'static str {
        "RegExp Match Indices (`hasIndices` / `d` flag)"
    }

    fn key(&self) -> &'static str {
        "es2022_reg_exp_match_indices_has_indices_d_flag"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2022 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-regexp-match-indices"
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
                name: "constructor supports it",
                exec: "return new RegExp('a', 'd') instanceof RegExp;",
            },
            Subtest {
                name: "shows up in flags",
                exec: "var expected = ['hasIndices'];\n// Sorted alphabetically by shortname – \"dgimsuy\".\nif ('global' in RegExp.prototype) expected.push('global');\nif ('ignoreCase' in RegExp.prototype) expected.push('ignoreCase');\nif ('multiline' in RegExp.prototype) expected.push('multiline');\nif ('dotAll' in RegExp.prototype) expected.push('dotAll');\nif ('unicode' in RegExp.prototype) expected.push('unicode');\nif ('sticky' in RegExp.prototype) expected.push('sticky');\nvar actual = [];\nvar p = new Proxy({}, { get: function (o, k) { actual.push(k); return o[k]; }});\nObject.getOwnPropertyDescriptor(RegExp.prototype, 'flags').get.call(p);\nif (expected.length !== actual.length) return false;\nfor (var i = 0; i < expected.length; i++) {\n  if (expected[i] !== actual[i]) return false;\n}\nreturn true;",
            },
        ]
    }
}
