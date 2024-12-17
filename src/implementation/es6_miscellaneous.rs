use crate::features::Es6Miscellaneous;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Miscellaneous {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
