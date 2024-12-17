use crate::features::Es2018RegExpLookbehindAssertions;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2018RegExpLookbehindAssertions {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
