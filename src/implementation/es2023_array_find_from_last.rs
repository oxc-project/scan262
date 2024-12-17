use crate::features::Es2023ArrayFindFromLast;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2023ArrayFindFromLast {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
