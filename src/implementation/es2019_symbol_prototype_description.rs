use crate::features::Es2019SymbolPrototypeDescription;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019SymbolPrototypeDescription {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
