use crate::feature::{Meta, Subtest};
use crate::features::EsnextClassAndPropertyDecorators;
impl Meta for EsnextClassAndPropertyDecorators {
    fn name(&self) -> &'static str {
        "Class and Property Decorators"
    }
    fn key(&self) -> &'static str {
        "esnext_class_and_property_decorators"
    }
    fn target(&self) -> &'static str {
        "esnext"
    }
    fn category(&self) -> &'static str {
        "Stage 3"
    }
    fn spec(&self) -> &'static str {
        "https://github.com/tc39/proposal-decorators"
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
            Subtest { name : "class decorators", exec :
            "class A {\n  @nonconf\n  get B() {}\n}\nfunction nonconf(target, name, descriptor) {\n  descriptor.configurable = false;\n  return descriptor;\n}\nreturn Object.getOwnPropertyDescriptor(A.prototype, \"B\").configurable === false;",
            },
        ]
    }
}
