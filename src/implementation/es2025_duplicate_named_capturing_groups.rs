use crate::features::Es2025DuplicateNamedCapturingGroups;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2025DuplicateNamedCapturingGroups {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
