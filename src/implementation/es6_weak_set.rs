use crate::features::Es6WeakSet;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6WeakSet {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}