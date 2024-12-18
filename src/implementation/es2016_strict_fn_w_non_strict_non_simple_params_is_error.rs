use crate::features::Es2016StrictFnWNonStrictNonSimpleParamsIsError;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2016StrictFnWNonStrictNonSimpleParamsIsError {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
