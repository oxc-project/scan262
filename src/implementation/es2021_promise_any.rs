use crate::features::Es2021PromiseAny;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2021PromiseAny {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
