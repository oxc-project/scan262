use crate::features::Es6ProxyInternalGetCalls;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProxyInternalGetCalls {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
