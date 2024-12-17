use crate::features::Es2017TrailingCommasInFunctionSyntax;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2017TrailingCommasInFunctionSyntax {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
