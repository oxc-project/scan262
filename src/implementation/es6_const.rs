use crate::features::Es6Const;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6Const {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
