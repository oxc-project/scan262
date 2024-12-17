use crate::features::Es6WellKnownSymbols;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6WellKnownSymbols {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}