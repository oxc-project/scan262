use crate::features::Es2022RegExpMatchIndicesHasIndicesDFlag;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2022RegExpMatchIndicesHasIndicesDFlag {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
