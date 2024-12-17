use crate::features::Es5ObjectStaticMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5ObjectStaticMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
