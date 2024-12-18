use oxc::semantic::{AstNode, Semantic};

use crate::{
    ctx::Ctx, feature::Feature,
    features::Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets,
};
impl Feature for Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
