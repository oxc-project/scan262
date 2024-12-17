use crate::features::Es6Class;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Class {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
