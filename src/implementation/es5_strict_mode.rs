use crate::features::Es5StrictMode;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5StrictMode {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
