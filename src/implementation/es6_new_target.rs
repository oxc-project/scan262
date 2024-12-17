use crate::features::Es6NewTarget;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6NewTarget {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
