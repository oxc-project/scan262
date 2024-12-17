use crate::features::Es2023ChangeArrayByCopy;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2023ChangeArrayByCopy {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
