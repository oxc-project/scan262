use crate::features::Es6ObjectStaticMethodsAcceptPrimitives;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ObjectStaticMethodsAcceptPrimitives {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
