use crate::features::Es6ObjectStaticMethodsAcceptPrimitives;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es6ObjectStaticMethodsAcceptPrimitives {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
