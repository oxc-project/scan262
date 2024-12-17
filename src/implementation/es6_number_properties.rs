use crate::features::Es6NumberProperties;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6NumberProperties {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
