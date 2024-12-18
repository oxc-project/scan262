use oxc::semantic::{AstNode, Semantic};

use crate::{
    ctx::Ctx, feature::Feature, features::Es2017AssignmentsAllowedInForInHeadInNonStrictMode,
};
impl Feature for Es2017AssignmentsAllowedInForInHeadInNonStrictMode {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
