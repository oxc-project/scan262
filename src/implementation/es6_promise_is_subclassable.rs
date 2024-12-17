use crate::features::Es6PromiseIsSubclassable;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6PromiseIsSubclassable {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
