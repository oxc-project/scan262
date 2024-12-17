use crate::features::Es6ArrowFunctions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ArrowFunctions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
