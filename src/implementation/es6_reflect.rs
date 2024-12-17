use crate::features::Es6Reflect;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Reflect {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
