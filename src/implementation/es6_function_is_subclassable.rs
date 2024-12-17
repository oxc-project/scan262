use crate::features::Es6FunctionIsSubclassable;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6FunctionIsSubclassable {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
