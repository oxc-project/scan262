use oxc::semantic::{AstNode, Semantic};
use crate::{feature::Feature, ctx::Ctx};
use crate::features::EsnextLegacyRegExpFeaturesInJavaScript;
impl Feature for EsnextLegacyRegExpFeaturesInJavaScript {
    fn test(&self, _node: &AstNode<'_>, _semantic: &Semantic<'_>, _ctx: &mut Ctx) {}
}
