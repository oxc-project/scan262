use crate::features::EsnextAsyncIteratorHelpers;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextAsyncIteratorHelpers {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
