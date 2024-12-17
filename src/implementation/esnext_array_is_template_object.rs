use crate::features::EsnextArrayIsTemplateObject;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextArrayIsTemplateObject {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
