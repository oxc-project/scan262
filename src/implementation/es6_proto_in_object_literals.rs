use crate::features::Es6ProtoInObjectLiterals;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6ProtoInObjectLiterals {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}