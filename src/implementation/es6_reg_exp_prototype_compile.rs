use crate::features::Es6RegExpPrototypeCompile;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RegExpPrototypeCompile {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
