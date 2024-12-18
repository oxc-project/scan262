use oxc::semantic::{AstNode, Semantic};

use crate::{ctx::Ctx, feature::Feature, features::Es2018RegExpUnicodePropertyEscapes};
impl Feature for Es2018RegExpUnicodePropertyEscapes {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
