use crate::features::EsnextClassAndPropertyDecorators;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextClassAndPropertyDecorators {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
