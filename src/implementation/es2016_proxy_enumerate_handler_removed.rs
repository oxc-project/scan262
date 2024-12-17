use crate::features::Es2016ProxyEnumerateHandlerRemoved;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2016ProxyEnumerateHandlerRemoved {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
