use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::EsnextUintArrayToFromBaseAndHex};
impl Feature for EsnextUintArrayToFromBaseAndHex {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
