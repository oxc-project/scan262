use crate::features::Es2019OptionalCatchBinding;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019OptionalCatchBinding {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
