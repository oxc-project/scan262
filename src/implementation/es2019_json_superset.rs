use crate::features::Es2019JsonSuperset;
use crate::{ctx::Ctx, feature::Feature};
use oxc::semantic::AstNode;
impl Feature for Es2019JsonSuperset {
    fn test(&self, _node: &AstNode<'_>, _ctx: &mut Ctx) {}
}
