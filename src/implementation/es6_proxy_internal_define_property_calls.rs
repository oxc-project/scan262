use crate::features::Es6ProxyInternalDefinePropertyCalls;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProxyInternalDefinePropertyCalls {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}