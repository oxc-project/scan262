use crate::features::Es6DestructuringDeclarations;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6DestructuringDeclarations {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
