use crate::features::Es2024PromiseWithResolvers;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2024PromiseWithResolvers {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
