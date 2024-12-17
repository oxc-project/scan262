use crate::features::Es2021LogicalAssignment;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2021LogicalAssignment {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
