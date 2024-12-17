use crate::features::Es6BlockLevelFunctionDeclaration;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6BlockLevelFunctionDeclaration {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
