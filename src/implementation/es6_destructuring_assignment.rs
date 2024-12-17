use crate::features::Es6DestructuringAssignment;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6DestructuringAssignment {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
