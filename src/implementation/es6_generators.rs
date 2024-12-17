use crate::features::Es6Generators;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Generators {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
