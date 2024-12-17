use crate::features::Es2019FunctionPrototypeToStringRevision;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019FunctionPrototypeToStringRevision {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
