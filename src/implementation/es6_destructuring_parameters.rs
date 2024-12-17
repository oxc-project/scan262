use crate::features::Es6DestructuringParameters;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6DestructuringParameters {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
