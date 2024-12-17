use crate::features::Es2017AsyncFunctions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2017AsyncFunctions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
