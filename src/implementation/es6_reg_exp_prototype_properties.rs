use crate::features::Es6RegExpPrototypeProperties;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es6RegExpPrototypeProperties {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
