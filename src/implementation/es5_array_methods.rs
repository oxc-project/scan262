use crate::features::Es5ArrayMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5ArrayMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
