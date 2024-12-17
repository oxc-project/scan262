use crate::features::Es5ObjectArrayLiteralExtensions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5ObjectArrayLiteralExtensions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
