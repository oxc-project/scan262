use crate::features::Es5ImmutableGlobals;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5ImmutableGlobals {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
