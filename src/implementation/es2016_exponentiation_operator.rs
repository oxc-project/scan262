use oxc::{
    ast::{ast::BinaryOperator, AstKind},
    semantic::{AstNode, Semantic},
};

use crate::{ctx::Ctx, feature::Feature, features::Es2016ExponentiationOperator};
impl Feature for Es2016ExponentiationOperator {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::BinaryExpression(expr) = node.kind() else { return };
        if expr.operator != BinaryOperator::Exponential {
            return;
        }
        ctx.diagnostic(self, expr.span);
    }
}
