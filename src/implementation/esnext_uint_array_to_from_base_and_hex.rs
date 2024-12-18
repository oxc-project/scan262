use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::EsnextUintArrayToFromBaseAndHex;
impl Feature for EsnextUintArrayToFromBaseAndHex {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
