use crate::features::Es6FunctionNameProperty;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6FunctionNameProperty {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
