use crate::features::Es2017ObjectStaticMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2017ObjectStaticMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
