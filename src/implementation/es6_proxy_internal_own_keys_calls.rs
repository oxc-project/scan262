use crate::features::Es6ProxyInternalOwnKeysCalls;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProxyInternalOwnKeysCalls {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
