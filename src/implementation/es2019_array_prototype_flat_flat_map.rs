use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2019ArrayPrototypeFlatFlatMap;
impl Feature for Es2019ArrayPrototypeFlatFlatMap {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
