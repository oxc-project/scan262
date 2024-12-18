use crate::feature::{Meta, Subtest};
use crate::features::Es2022StaticClassFields;
impl Meta for Es2022StaticClassFields {
    fn name(&self) -> &'static str {
        "static class fields"
    }
    fn key(&self) -> &'static str {
        "es2022_static_class_fields"
    }
    fn target(&self) -> &'static str {
        "es2016"
    }
    fn category(&self) -> &'static str {
        "2022 features"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-static-class-features/"
    }
    fn significance(&self) -> &'static str {
        "medium"
    }
    fn mdn(&self) -> &'static str {
        ""
    }
    fn exec(&self) -> &'static str {
        ""
    }
    fn subtests(&self) -> Vec<Subtest> {
        vec![
            Subtest { name : "public static class fields", exec :
            "class C {\n  static x = 'x';\n}\nreturn C.x === 'x';", }, Subtest { name :
            "static class fields use [[Define]]", exec :
            "return (class X { static name = \"name\"; }).name === 'name';", }, Subtest {
            name : "private static class fields", exec :
            "class C {\n  static #x = 42;\n  x() {\n    return C.#x;\n  }\n}\nreturn new C().x() === 42;",
            }, Subtest { name : "computed static class fields", exec :
            "class C {\n  static ['x'] = 42;\n}\nreturn C.x === 42;", },
        ]
    }
}
