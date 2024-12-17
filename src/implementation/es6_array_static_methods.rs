use crate::features::Es6ArrayStaticMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ArrayStaticMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
