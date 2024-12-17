use crate::features::Es6Set;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Set {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
