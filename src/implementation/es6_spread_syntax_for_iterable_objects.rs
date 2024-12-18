use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es6SpreadSyntaxForIterableObjects;
impl Feature for Es6SpreadSyntaxForIterableObjects {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
