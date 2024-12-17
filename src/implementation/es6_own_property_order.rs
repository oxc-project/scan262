use crate::features::Es6OwnPropertyOrder;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6OwnPropertyOrder {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
