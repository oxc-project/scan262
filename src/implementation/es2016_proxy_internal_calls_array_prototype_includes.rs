use crate::features::Es2016ProxyInternalCallsArrayPrototypeIncludes;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2016ProxyInternalCallsArrayPrototypeIncludes {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
