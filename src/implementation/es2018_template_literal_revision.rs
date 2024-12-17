use crate::features::Es2018TemplateLiteralRevision;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018TemplateLiteralRevision {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
