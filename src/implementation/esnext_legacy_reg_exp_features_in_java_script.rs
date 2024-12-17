use crate::features::EsnextLegacyRegExpFeaturesInJavaScript;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for EsnextLegacyRegExpFeaturesInJavaScript {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
