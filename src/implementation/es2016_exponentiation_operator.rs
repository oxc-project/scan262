use crate::features::Es2016ExponentiationOperator;
use crate::{ctx::Ctx, feature::Feature};
use oxc::ast::ast::BinaryOperator;
use oxc::ast::AstKind;
use oxc::semantic::{AstNode, Semantic};
impl Feature for Es2016ExponentiationOperator {
    fn test(&self, node: &AstNode<'_>, _semantic: &Semantic<'_>, ctx: &mut Ctx) {
        let AstKind::BinaryExpression(expr) = node.kind() else { return };
        if expr.operator != BinaryOperator::Exponential {
            return;
        }
        ctx.diagnostic(self, expr.span);
    }
}
