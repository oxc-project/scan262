use crate::features::Es6WeakMap;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6WeakMap {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}