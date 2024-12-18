use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2024ArrayBufferPrototypeTransfer};
impl Feature for Es2024ArrayBufferPrototypeTransfer {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
