use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es6DatePrototypeSymbolToPrimitive;
impl Feature for Es6DatePrototypeSymbolToPrimitive {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
