use crate::features::Es2020NullishCoalescingOperator;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2020NullishCoalescingOperator {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
