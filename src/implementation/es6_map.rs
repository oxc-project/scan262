use crate::features::Es6Map;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Map {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
