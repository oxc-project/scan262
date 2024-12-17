use crate::features::Es2016GeneratorFunctionsCanTBeUsedWithNew;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2016GeneratorFunctionsCanTBeUsedWithNew {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
