use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2025IteratorHelpers};
impl Feature for Es2025IteratorHelpers {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
