use crate::features::Es6DefaultFunctionParameters;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6DefaultFunctionParameters {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
