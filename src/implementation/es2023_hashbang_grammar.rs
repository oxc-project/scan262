use crate::features::Es2023HashbangGrammar;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2023HashbangGrammar {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
