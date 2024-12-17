use crate::features::Es6ObjectLiteralExtensions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ObjectLiteralExtensions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
