use crate::features::Es6RegExpSyntaxExtensions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RegExpSyntaxExtensions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
