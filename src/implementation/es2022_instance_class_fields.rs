use crate::features::Es2022InstanceClassFields;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022InstanceClassFields {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
