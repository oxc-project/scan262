use crate::features::Es6ProxyInternalDeletePropertyCalls;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProxyInternalDeletePropertyCalls {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
