use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2016ProxyEnumerateHandlerRemoved;
impl Feature for Es2016ProxyEnumerateHandlerRemoved {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
