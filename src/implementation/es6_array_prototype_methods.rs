use crate::features::Es6ArrayPrototypeMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ArrayPrototypeMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}