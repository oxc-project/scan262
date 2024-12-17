use crate::features::Es6RegExpIsSubclassable;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RegExpIsSubclassable {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
