use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es6ProxyInternalGetCalls;
impl Feature for Es6ProxyInternalGetCalls {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
