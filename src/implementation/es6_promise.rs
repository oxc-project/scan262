use crate::features::Es6Promise;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Promise {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
