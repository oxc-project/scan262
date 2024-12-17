use crate::features::Es2018SDotAllFlagForRegularExpressions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018SDotAllFlagForRegularExpressions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
