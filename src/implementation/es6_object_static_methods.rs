use crate::features::Es6ObjectStaticMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ObjectStaticMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
