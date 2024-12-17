use crate::features::Es6StringPrototypeMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6StringPrototypeMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
