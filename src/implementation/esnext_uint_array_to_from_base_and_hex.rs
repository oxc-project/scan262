use crate::features::EsnextUintArrayToFromBaseAndHex;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextUintArrayToFromBaseAndHex {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
