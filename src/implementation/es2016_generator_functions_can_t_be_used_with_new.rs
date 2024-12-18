use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2016GeneratorFunctionsCanTBeUsedWithNew;
impl Feature for Es2016GeneratorFunctionsCanTBeUsedWithNew {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
