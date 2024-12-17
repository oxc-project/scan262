use crate::features::Es5FunctionPrototypeBind;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5FunctionPrototypeBind {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
