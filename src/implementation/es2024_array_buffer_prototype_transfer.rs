use crate::features::Es2024ArrayBufferPrototypeTransfer;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2024ArrayBufferPrototypeTransfer {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
