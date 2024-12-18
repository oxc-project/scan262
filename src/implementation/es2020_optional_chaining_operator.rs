use crate::features::Es2020OptionalChainingOperator;
use crate::{ctx::Ctx, feature::Feature};
use oxc::{
    ast::AstKind,
    semantic::{AstNode, Semantic},
};
impl Feature for Es2020OptionalChainingOperator {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::ChainExpression(expr) = node.kind() else { return };
        ctx.diagnostic(self, expr.span);
    }
}
