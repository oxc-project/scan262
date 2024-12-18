use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2022AtMethodOnTheBuiltInIndexables};
impl Feature for Es2022AtMethodOnTheBuiltInIndexables {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
