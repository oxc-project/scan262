use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2021LogicalAssignment};
impl Feature for Es2021LogicalAssignment {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
