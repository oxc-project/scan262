use crate::features::Es2017ObjectPrototypeGetterSetterMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2017ObjectPrototypeGetterSetterMethods {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
