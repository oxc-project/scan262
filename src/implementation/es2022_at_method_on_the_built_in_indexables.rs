use crate::features::Es2022AtMethodOnTheBuiltInIndexables;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022AtMethodOnTheBuiltInIndexables {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}