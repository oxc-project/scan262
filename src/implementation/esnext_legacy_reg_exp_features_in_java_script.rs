use crate::features::EsnextLegacyRegExpFeaturesInJavaScript;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::{AstNode, Semantic};
impl Feature for EsnextLegacyRegExpFeaturesInJavaScript {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
