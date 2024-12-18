use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2016GeneratorThrowCaughtByInnerGenerator};
impl Feature for Es2016GeneratorThrowCaughtByInnerGenerator {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
