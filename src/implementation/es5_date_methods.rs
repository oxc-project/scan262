use crate::features::Es5DateMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5DateMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
