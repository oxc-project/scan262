use crate::features::Es2025RegExpPatternModifiers;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2025RegExpPatternModifiers {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
