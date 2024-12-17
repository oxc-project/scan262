use crate::features::Es6Let;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Let {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
