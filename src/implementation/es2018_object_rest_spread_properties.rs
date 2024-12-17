use crate::features::Es2018ObjectRestSpreadProperties;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018ObjectRestSpreadProperties {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
