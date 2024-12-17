use crate::features::Es6StringStaticMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6StringStaticMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
