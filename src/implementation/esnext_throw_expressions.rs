use crate::features::EsnextThrowExpressions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextThrowExpressions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
