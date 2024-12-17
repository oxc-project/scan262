use crate::features::Es5NumberMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5NumberMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
