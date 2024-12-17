use crate::features::Es6RestParameters;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RestParameters {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
