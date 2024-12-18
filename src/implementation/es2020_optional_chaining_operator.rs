use oxc::{
    ast::AstKind,
    semantic::{AstNode, Semantic},
};

use crate::{ctx::Ctx, feature::Feature, features::Es2020OptionalChainingOperator};
impl Feature for Es2020OptionalChainingOperator {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::ChainExpression(expr) = node.kind() else { return };
        ctx.diagnostic(self, expr.span);
    }
}
