use crate::features::Es6RegExpYAndUFlags;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RegExpYAndUFlags {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
