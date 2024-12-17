use crate::features::Es5StringPropertiesAndMethods;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es5StringPropertiesAndMethods {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
