use crate::features::Es6ObjectPrototypeProto;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es6ObjectPrototypeProto {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
