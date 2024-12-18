use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2016StrictFnWNonStrictNonSimpleParamsIsError};
impl Feature for Es2016StrictFnWNonStrictNonSimpleParamsIsError {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
