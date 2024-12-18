use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2017TrailingCommasInFunctionSyntax};
impl Feature for Es2017TrailingCommasInFunctionSyntax {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
