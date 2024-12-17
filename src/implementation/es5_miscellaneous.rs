use crate::features::Es5Miscellaneous;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5Miscellaneous {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
