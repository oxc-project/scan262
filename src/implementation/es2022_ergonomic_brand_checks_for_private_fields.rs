use crate::features::Es2022ErgonomicBrandChecksForPrivateFields;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2022ErgonomicBrandChecksForPrivateFields {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
