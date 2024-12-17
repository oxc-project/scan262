use crate::features::Es2018PromisePrototypeFinally;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018PromisePrototypeFinally {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
