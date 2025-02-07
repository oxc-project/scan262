use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2018PromisePrototypeFinally};
impl Feature for Es2018PromisePrototypeFinally {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
