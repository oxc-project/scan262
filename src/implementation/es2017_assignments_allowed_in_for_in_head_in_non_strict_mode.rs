use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2017AssignmentsAllowedInForInHeadInNonStrictMode;
impl Feature for Es2017AssignmentsAllowedInForInHeadInNonStrictMode {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
