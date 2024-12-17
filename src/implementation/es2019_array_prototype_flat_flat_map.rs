use crate::features::Es2019ArrayPrototypeFlatFlatMap;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019ArrayPrototypeFlatFlatMap {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
