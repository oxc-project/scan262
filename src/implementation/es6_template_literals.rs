use crate::features::Es6TemplateLiterals;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6TemplateLiterals {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
