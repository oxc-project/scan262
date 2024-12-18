use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2019ArrayPrototypeFlatFlatMap};
impl Feature for Es2019ArrayPrototypeFlatFlatMap {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
