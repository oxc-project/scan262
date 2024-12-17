use crate::features::Es6ArrayIsSubclassable;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ArrayIsSubclassable {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
