use crate::features::Es6TypedArrays;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6TypedArrays {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
