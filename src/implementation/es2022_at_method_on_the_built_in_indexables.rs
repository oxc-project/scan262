use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2022AtMethodOnTheBuiltInIndexables;
impl Feature for Es2022AtMethodOnTheBuiltInIndexables {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
