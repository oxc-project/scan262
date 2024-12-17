use crate::features::Es5Json;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5Json {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
