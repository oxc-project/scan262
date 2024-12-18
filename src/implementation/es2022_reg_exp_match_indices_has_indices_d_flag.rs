use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2022RegExpMatchIndicesHasIndicesDFlag;
impl Feature for Es2022RegExpMatchIndicesHasIndicesDFlag {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
