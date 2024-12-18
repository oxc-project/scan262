use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es6ProxyInternalGetOwnPropertyDescriptorCalls};
impl Feature for Es6ProxyInternalGetOwnPropertyDescriptorCalls {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
