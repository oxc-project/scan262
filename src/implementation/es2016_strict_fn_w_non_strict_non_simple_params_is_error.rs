use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::Es2016StrictFnWNonStrictNonSimpleParamsIsError;
impl Feature for Es2016StrictFnWNonStrictNonSimpleParamsIsError {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
