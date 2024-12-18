use crate::features::Es2017AssignmentsAllowedInForInHeadInNonStrictMode;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2017AssignmentsAllowedInForInHeadInNonStrictMode {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
