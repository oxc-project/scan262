use crate::features::Es6UnicodeCodePointEscapes;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6UnicodeCodePointEscapes {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
