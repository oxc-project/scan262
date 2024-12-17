use crate::features::Es2022StaticClassFields;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022StaticClassFields {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
