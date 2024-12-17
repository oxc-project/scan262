use crate::features::EsnextMapPrototypeUpsert;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextMapPrototypeUpsert {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
