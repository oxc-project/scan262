use crate::{
    feature::{Meta, Subtest},
    features::Es2022ErgonomicBrandChecksForPrivateFields,
};
impl Meta for Es2022ErgonomicBrandChecksForPrivateFields {
    fn name(&self) -> &'static str {
        "Ergonomic brand checks for private fields"
    }

    fn key(&self) -> &'static str {
        "es2022_ergonomic_brand_checks_for_private_fields"
    }

    fn target(&self) -> &'static str {
        "es2016"
    }

    fn category(&self) -> &'static str {
        "2022 features"
    }

    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-private-fields-in-in"
    }

    fn significance(&self) -> &'static str {
        "small"
    }

    fn mdn(&self) -> &'static str {
        ""
    }

    fn exec(&self) -> &'static str {
        "class A {\n  #x;\n  static check(obj) {\n    return #x in obj;\n  }\n}\nreturn A.check(new A) && !A.check({});"
    }

    fn subtests(&self) -> Vec<Subtest> {
        vec![]
    }
}
