use crate::features::Es2020BigInt;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2020BigInt {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
