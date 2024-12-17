use crate::features::Es6MiscellaneousSubclassables;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6MiscellaneousSubclassables {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
