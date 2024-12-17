use crate::features::Es2016GeneratorThrowCaughtByInnerGenerator;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2016GeneratorThrowCaughtByInnerGenerator {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
