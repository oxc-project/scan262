use crate::features::Es6NonStrictFunctionSemantics;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6NonStrictFunctionSemantics {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
