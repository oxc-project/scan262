use crate::features::Es6Super;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Super {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
