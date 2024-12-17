use crate::features::EsnextGeneratorFunctionSentMetaProperty;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextGeneratorFunctionSentMetaProperty {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
