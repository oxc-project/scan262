use crate::features::Es6ProperTailCallsTailCallOptimisation;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProperTailCallsTailCallOptimisation {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
