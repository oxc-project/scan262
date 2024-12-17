use crate::features::Es6ForOfLoops;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ForOfLoops {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
