use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2016GeneratorThrowCaughtByInnerGenerator;
impl Feature for Es2016GeneratorThrowCaughtByInnerGenerator {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
