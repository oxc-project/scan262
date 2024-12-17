use crate::features::Es2022ClassStaticInitializationBlocks;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022ClassStaticInitializationBlocks {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
