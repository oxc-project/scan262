use crate::features::Es6StringPrototypeHtmlMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6StringPrototypeHtmlMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
