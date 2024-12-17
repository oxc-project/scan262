use crate::features::Es2025PromiseTry;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2025PromiseTry {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
