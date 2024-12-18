use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2022RegExpMatchIndicesHasIndicesDFlag};
impl Feature for Es2022RegExpMatchIndicesHasIndicesDFlag {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
