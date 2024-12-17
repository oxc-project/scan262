use crate::features::EsnextRegExpEscaping;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextRegExpEscaping {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
