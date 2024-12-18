use crate::features::Es6ProxyInternalGetOwnPropertyDescriptorCalls;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es6ProxyInternalGetOwnPropertyDescriptorCalls {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
