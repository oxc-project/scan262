use crate::features::Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018ProxyOwnKeysHandlerDuplicateKeysForNonExtensibleTargets {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
