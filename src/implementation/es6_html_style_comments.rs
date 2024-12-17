use crate::features::Es6HtmlStyleComments;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6HtmlStyleComments {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
