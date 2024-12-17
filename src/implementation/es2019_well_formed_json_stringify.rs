use crate::features::Es2019WellFormedJsonStringify;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019WellFormedJsonStringify {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
