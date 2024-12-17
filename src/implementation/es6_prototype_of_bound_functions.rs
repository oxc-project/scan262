use crate::features::Es6PrototypeOfBoundFunctions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6PrototypeOfBoundFunctions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
