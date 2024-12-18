use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es6Set;
impl Feature for Es6Set {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
