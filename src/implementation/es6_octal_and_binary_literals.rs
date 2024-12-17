use crate::features::Es6OctalAndBinaryLiterals;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6OctalAndBinaryLiterals {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
