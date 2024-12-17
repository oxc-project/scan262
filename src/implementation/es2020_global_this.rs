use crate::features::Es2020GlobalThis;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2020GlobalThis {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
