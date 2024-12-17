use crate::features::Es2017StringPadding;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2017StringPadding {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
