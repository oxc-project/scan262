use crate::features::Es2022PrivateClassMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022PrivateClassMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}