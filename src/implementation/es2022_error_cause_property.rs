use crate::features::Es2022ErrorCauseProperty;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022ErrorCauseProperty {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
