use crate::features::Es2021StringPrototypeReplaceAll;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2021StringPrototypeReplaceAll {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}