use crate::features::EsnextShadowRealm;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextShadowRealm {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
