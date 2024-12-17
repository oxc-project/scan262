use crate::features::Es6DatePrototypeSymbolToPrimitive;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6DatePrototypeSymbolToPrimitive {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
